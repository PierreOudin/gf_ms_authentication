use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, ModelTrait};
use user::{user_service_server::UserService, *};
use tonic::{Request, Response, Status};
use entity::{address, Address, User};
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
        println!("Got a request: {:?}", request);

        let request = request.into_inner();

        let user = User::find()
            .filter(entity::user::Column::Email.eq(&request.email))
            .one(&self.db)
            .await.map_err(|err| Status::internal(err.to_string()))?;

        match user {
            Some(_) => {
                return Ok(Response::new(AddUserResponse {
                    status: false,
                    message: "alreadyreported".to_lowercase().to_string(),
                    ..Default::default()
                }))
            }
            None => {
                let user_model = entity::user::ActiveModel {
                    firstname: Set(request.firstname).to_owned(),
                    lastname: Set(request.lastname).to_owned(),
                    email: Set(request.email).to_owned(),
                    phone: Set(request.phone).to_owned(),
                    password: Set(request.password).to_owned(),
                    id_restaurant: Set(None).to_owned(),
                    ..Default::default()
                };
                let result = User::insert(user_model).exec(&self.db).await.map_err(|err| Status::internal(err.to_string()))?;
                let response = user::AddUserResponse {
                    status: true,
                    message: "success".to_string(),
                    id: result.last_insert_id as u32,
                };

                Ok(Response::new(response))
            }
            
        }
    }

    async fn delete_user_by_id(
        &self,
        request: Request<DeleteUserByIdRequest>,
    ) -> Result<Response<DeleteUserByIdResponse>, Status> {
        println!("Got a request: {:?}", request);

        let request = request.into_inner();

        let deleted: sea_orm::DeleteResult = User::delete(entity::user::ActiveModel {
            id: Set(request.id as i32).to_owned(),
            ..Default::default()
        }).exec(&self.db).await.map_err(|err| Status::internal(err.to_string()))?;

        match deleted.rows_affected == 1 {
            true => Ok(Response::new(DeleteUserByIdResponse {
                status: true,
                ..Default::default()
            })),
            false => Ok(Response::new(DeleteUserByIdResponse {
                status: false,
                message: "internalerror".to_lowercase().to_string(),
                ..Default::default()
            }))
        }
    }

    async fn get_users(
        &self,
        _: Request<GetUsersRequest>,
    ) -> Result<Response<GetUsersResponse>, Status> {
        
        let users = User::find()
            .all(&self.db)
            .await.map_err(|err| Status::internal(err.to_string()))?;

        let users: Vec<UserItem> = match users.len() > 0 {
            true => {
                users.iter().map(|user| {
                    UserItem {
                        id: user.id as u32,
                        firstname: user.firstname.clone(),
                        lastname: user.lastname.clone(),
                        email: user.email.clone(),
                        phone: user.phone.clone(),
                    }
                }).collect()
            },
            false => return Ok(Response::new(GetUsersResponse{
                status: false,
                message: "nocontent".to_lowercase().to_string(),
                ..Default::default()
            }))
        };

        let response = user::GetUsersResponse {
            status: true,
            users: users,
            message: "success".to_string(),
            ..Default::default()
        };

        Ok(Response::new(response))
    }

    async fn get_user_by_id(
        &self,
        request: Request<GetUserByIdRequest>,
    ) -> Result<Response<GetUserByIdResponse>, Status> {
        println!("Got a request: {:?}", request);

        let request = request.into_inner();

        let user = User::find()
            .filter(entity::user::Column::Id.eq(request.id as i32))
            .one(&self.db)
            .await.map_err(|err| Status::internal(err.to_string()))?;

        let Some(u) = user else {
            return Ok(Response::new(GetUserByIdResponse {
                status: false,
                message: "notfound".to_lowercase().to_string(),
                ..Default::default()
            }))
        };

        let response = user::GetUserByIdResponse {
            status: true,
            message: "success".to_string(),
            firstname: u.firstname.clone(),
            lastname: u.lastname.clone(),
            email: u.email.clone(),
            phone: u.phone.clone(),
        };

        Ok(Response::new(response))
    }

    async fn update_user_by_id(
        &self,
        request: Request<UpdateUserByIdRequest>,
    ) -> Result<Response<UpdateUserByIdResponse>, Status> {
        println!("Got a request: {:?}", request);

        let request = request.into_inner();

        let user = User::find()
            .filter(entity::user::Column::Id.eq(request.id as i32))
            .one(&self.db)
            .await.map_err(|err| Status::internal(err.to_string()))?;

        let mut user: entity::user::ActiveModel = match user {
            Some(u) => u.into(),
            None => return Ok(Response::new(UpdateUserByIdResponse {
                status: false,
                message: "notfound".to_lowercase().to_string(),
                ..Default::default()
            }))
        };

        user.firstname = Set(request.firstname.to_owned());
        user.lastname = Set(request.lastname.to_owned());
        user.email = Set(request.email.to_owned());
        user.phone = Set(request.phone.to_owned());

        let user: entity::user::Model = user.update(&self.db).await.map_err(|err| Status::internal(err.to_string()))?;

        // let updated: sea_orm::UpdateResult = User::update()
        //     .set(entity::user::Column::Firstname, Set(request.firstname).to_owned())
        //     .set(entity::user::Column::Lastname, Set(request.lastname).to_owned())
        //     .set(entity::user::Column::Email, Set(request.email).to_owned())
        //     .set(entity::user::Column::Phone, Set(request.phone).to_owned())
        //     .filter(entity::user::Column::Id.eq(request.id as i32))
        //     .exec(&self.db)
        //     .await.map_err(|err| Status::internal(err.to_string()))?;
        // Implement the update_user_by_id method here

        let response = user::UpdateUserByIdResponse {
            status: true,
            message: "success".to_string(),
            firstname: user.firstname.clone(),
            lastname: user.lastname.clone(),
            email : user.email.clone(),
            phone : user.phone.clone(),
        };

        Ok(Response::new(response))
    }

    async fn update_password_by_user_id(
        &self,
        request: Request<UpdatePasswordByUserIdRequest>,
    ) -> Result<Response<UpdatePasswordByUserIdResponse>, Status> {
        println!("Got a request: {:?}", request);

        let request = request.into_inner();

        let user = User::find()
            .filter(entity::user::Column::Id.eq(request.id_user as i32))
            .one(&self.db)
            .await.map_err(|err| Status::internal(err.to_string()))?;

        let mut user: entity::user::ActiveModel = match user {
            Some(u) => u.into(),
            None => return Ok(Response::new(UpdatePasswordByUserIdResponse {
                status: false,
                message: "notfound".to_lowercase().to_string(),
                ..Default::default()
            }))
        };

        user.password = Set(request.password.to_owned());

        let _: entity::user::Model = user.update(&self.db).await.map_err(|err| Status::internal(err.to_string()))?;

        let response = user::UpdatePasswordByUserIdResponse {
            status: true,
            message: "success".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn get_addresses_by_user_id(
        &self,
        request: Request<GetAddressesByUserIdRequest>,
    ) -> Result<Response<GetAddressesByUserIdResponse>, Status> {
        println!("Got a request: {:?}", request);

        let request = request.into_inner();

        let user = User::find()
            .filter(entity::user::Column::Id.eq(request.id as i32))
            .one(&self.db)
            .await.map_err(|err| Status::internal(err.to_string()))?;

        let user: entity::user::Model = match user {
            Some(u) => u,
            None => return Ok(Response::new(GetAddressesByUserIdResponse {
                status: false,
                message: "notfound".to_lowercase().to_string(),
                ..Default::default()
            }))
        };

        let addresses: Vec<address::Model> = user.find_related(Address)
            .all(&self.db)
            .await.map_err(|err| Status::internal(err.to_string()))?;

        let addresses: Vec<user::Address> = match addresses.len() > 0 {
            true => {
                addresses.iter().map(|address| {
                    user::Address {
                        id: address.id as u32,
                        address: address.address.clone(),
                        city: address.city.clone(),
                        cp: address.zip.clone(),
                    }
                }).collect()
            },
            false => return Ok(Response::new(GetAddressesByUserIdResponse{
                status: false,
                message: "nocontent".to_lowercase().to_string(),
                ..Default::default()
            }))
        };

        let response = user::GetAddressesByUserIdResponse {
            status: true,
            addresses: addresses,
            message: "success".to_string(),
            ..Default::default()
        };

        Ok(Response::new(response))
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