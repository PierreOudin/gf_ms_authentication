use tonic::{transport::Server, Request, Response, Status};

use authentication::authenticated_service_server::{AuthenticatedService, AuthenticatedServiceServer};
use authentication::{LoginRequest, LoginResponse, VerifyTokenRequest, VerifyTokenResponse};

pub mod authentication {
    tonic::include_proto!("_");
}

#[derive(Debug, Default)]
pub struct MyAuthentication {}

#[tonic::async_trait]
impl AuthenticatedService for MyAuthentication {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = authentication::LoginResponse {
            status: true,
            token: "token".to_string(),
            message: "".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn verify_token(
        &self,
        request: Request<VerifyTokenRequest>,
    ) -> Result<Response<VerifyTokenResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = authentication::VerifyTokenResponse {
            status: true,
            message: "".to_string(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let authentication = MyAuthentication::default();

    Server::builder()
        .add_service(AuthenticatedServiceServer::new(authentication))
        .serve(addr)
        .await?;

    Ok(())
}