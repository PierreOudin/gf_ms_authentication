use tonic::{Request, Response, Status};

use authentication::{LoginRequest, LoginResponse, VerifyTokenRequest, VerifyTokenResponse, authenticated_service_server::AuthenticatedService};

pub mod authentication {
    include!("../generated/protobuf/authentication.rs");
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