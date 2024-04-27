

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Set};
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

        let user = User::find()
        .filter(user::Column::Email.eq(&request.username))
        .one(&self.db)
        .await.map_err(|err| Status::internal(err.to_string()))?;

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
                message: "internalerror".to_string().to_lowercase(),
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

        let request = request.into_inner();

        let claims = match jwt::generate::validate_token(request.token) {
            Ok(c) => Some(c),
            Err(e) => return Ok(Response::new(VerifyTokenResponse{
                    status: false,
                    message: e.to_string().replace(" ", "").to_lowercase(),
                })),
        };

        let Some(c) = claims else { return Ok(Response::new(VerifyTokenResponse{
            status: false,
            message: "invalidtoken".to_lowercase().to_string(),
        })) };
        
        let user = User::find()
        .filter(user::Column::Id.eq(c.sub))
        .filter(user::Column::Email.eq(c.email))
        .filter(user::Column::Firstname.eq(c.fistname))
        .filter(user::Column::Lastname.eq(c.lastname))
        .one(&self.db)
        .await.map_err(|err| Status::internal(err.to_string()))?;

        match user {
            Some(_) => return Ok(Response::new(authentication::VerifyTokenResponse {
                status: true,
                message: "".to_string(),
            })),
            None => return Ok(Response::new(authentication::VerifyTokenResponse {
                status: false,
                message: "invalidtoken".to_string(),
            })),
        };
    }

    async fn sign_up(
        &self,
        request: Request<SignUpRequest>,
    ) -> Result<Response<SignUpResponse>, Status> {
        println!("Got a request: {:?}", request);
        let request = request.into_inner();

        let user = User::find()
        .filter(user::Column::Email.eq(&request.email))
        .select_only().column(user::Column::Email)
        .one(&self.db)
        .await.map_err(|err| Status::internal(err.to_string()))?;

        let last_id = match user {
            Some(_) => return Ok(Response::new(authentication::SignUpResponse {
                status: false,
                message: "alreadyreported".to_string(),
                id: 0,
            })),
            None => {
                let user_model = user::ActiveModel {
                    firstname: Set(request.firstname).to_owned(),
                    lastname: Set(request.lastname).to_owned(),
                    email: Set(request.email).to_owned(),
                    phone: Set(request.phone).to_owned(),
                    password: Set(request.password).to_owned(),
                    id_restaurant: Set(None).to_owned(),
                    ..Default::default()
                };
                let result = User::insert(user_model).exec(&self.db).await.map_err(|err| Status::internal(err.to_string()))?;
                result.last_insert_id
            }
        };

        let response = authentication::SignUpResponse {
            status: true,
            message: "created".to_string(),
            id: last_id as u32,
        };

        Ok(Response::new(response))
    }
}