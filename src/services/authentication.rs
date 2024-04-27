

use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use tonic::{Request, Response, Status};

use authentication::{LoginRequest, LoginResponse, VerifyTokenRequest, VerifyTokenResponse, SignUpRequest, SignUpResponse, authenticated_service_server::AuthenticatedService};
use entity::{user, User};

use crate::jwt;

pub mod authentication {
    include!("../generated/protobuf/authentication.rs");
}

#[derive(Debug, Default)]
pub struct MyAuthentication {
    pub db: DatabaseConnection
}

impl MyAuthentication {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db: db
        }
    }
}

#[tonic::async_trait]
impl AuthenticatedService for MyAuthentication {
    
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        println!("Got a request: {:?}", request);

        let request = request.into_inner();

        let user = User::find().filter(user::Column::Email.eq(&request.username)).one(&self.db).await.map_err(|err| Status::internal(err.to_string()))?;

        let user = if let Some(u) = user {
            u
        }else{
            return Ok(Response::new(LoginResponse{
                        status: false,
                        token: "".to_string(),
                        message: "not_found".to_lowercase().to_string(),
                    }))
        }; 

        if user.password == request.password {
            return Ok(Response::new(LoginResponse{
                status: false,
                token: "".to_string(),
                message: "unauthorized".to_lowercase().to_string(),
            }))
        }

        let token = match jwt::generate::create_token(user) {
            Ok(t) => Some(t),
            Err(e) => return Ok(Response::new(LoginResponse{
                    status: false,
                    token: "".to_string(),
                    message: e.to_string().to_lowercase(),
                })),
        };

        let token = match token {
            Some(t) => t,
            None => return Ok(Response::new(LoginResponse{
                status: false,
                token: "".to_string(),
                message: "internalerror".to_lowercase(),
            }))
        };

        let response = authentication::LoginResponse {
            status: true,
            token: token.to_string(),
            message: "success".to_string(),
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

    async fn sign_up(
        &self,
        request: Request<SignUpRequest>,
    ) -> Result<Response<SignUpResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = authentication::SignUpResponse{
            status: true,
            id: 1,
        };

        Ok(Response::new(response))
    }
}