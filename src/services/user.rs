use sea_orm::DatabaseConnection;
use user::{user_service_server::UserService, *};
use tonic::{Request, Response, Status};
pub mod user {
    include!("../generated/protobuf/user.rs");
}

#[derive(Debug, Default)]
pub struct MyUser {
    pub db: DatabaseConnection
}

impl MyUser {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db: db
        }
    }
}

#[tonic::async_trait]
impl UserService for MyUser {
    async fn add_user(
        &self,
        request: Request<AddUserRequest>,
    ) -> Result<Response<AddUserResponse>, Status> {
        // Implement the add_user method here
        unimplemented!()
    }

    async fn delete_user_by_id(
        &self,
        request: Request<DeleteUserByIdRequest>,
    ) -> Result<Response<DeleteUserByIdResponse>, Status> {
        // Implement the delete_user_by_id method here
        unimplemented!()
    }

    async fn get_users(
        &self,
        request: Request<GetUsersRequest>,
    ) -> Result<Response<GetUsersResponse>, Status> {
        // Implement the get_users method here
        unimplemented!()
    }

    async fn get_user_by_id(
        &self,
        request: Request<GetUserByIdRequest>,
    ) -> Result<Response<GetUserByIdResponse>, Status> {
        // Implement the get_user_by_id method here
        unimplemented!()
    }

    async fn update_user_by_id(
        &self,
        request: Request<UpdateUserByIdRequest>,
    ) -> Result<Response<UpdateUserByIdResponse>, Status> {
        // Implement the update_user_by_id method here
        unimplemented!()
    }

    async fn update_password_by_user_id(
        &self,
        request: Request<UpdatePasswordByUserIdRequest>,
    ) -> Result<Response<UpdatePasswordByUserIdResponse>, Status> {
        // Implement the update_password_by_user_id method here
        unimplemented!()
    }

    async fn get_addresses_by_user_id(
        &self,
        request: Request<GetAddressesByUserIdRequest>,
    ) -> Result<Response<GetAddressesByUserIdResponse>, Status> {
        // Implement the get_addresses_by_user_id method here
        unimplemented!()
    }

    async fn remove_address_by_id_to_user(
        &self,
        request: Request<RemoveAddressByIdRequest>,
    ) -> Result<Response<RemoveAddressByIdResponse>, Status> {
        // Implement the remove_address_by_id_to_user method here
        unimplemented!()
    }

    async fn update_address_by_id(
        &self,
        request: Request<UpdateAddressByIdRequest>,
    ) -> Result<Response<UpdateAddressByIdResponse>, Status> {
        // Implement the update_address_by_id method here
        unimplemented!()
    }

    async fn set_default_address_to_user(
        &self,
        request: Request<SetDefaultAdrToUserRequest>,
    ) -> Result<Response<SetDefaultAdrToUserResponse>, Status> {
        // Implement the set_default_address_to_user method here
        unimplemented!()
    }

    async fn get_default_address_to_user(
        &self,
        request: Request<GetDefaultAdrToUserRequest>,
    ) -> Result<Response<GetDefaultAdrToUserResponse>, Status> {
        // Implement the get_default_address_to_user method here
        unimplemented!()
    }
}