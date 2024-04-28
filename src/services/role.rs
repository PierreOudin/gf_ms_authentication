use role::role_service_server::RoleService;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set};
use tonic::{Request, Response, Status};
use entity::{user, usr_asso_rol, Role, User, UsrAssoRol};

use self::role::*;
pub mod role {
    include!("../generated/protobuf/role.rs");
}

#[derive(Debug, Default)]
pub struct MyRole {
    pub db: DatabaseConnection
}

impl MyRole {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db: db
        }
    }
}

#[tonic::async_trait]
impl RoleService for MyRole{

    async fn get_role(
        &self,
        request: Request<GetRoleRequest>,
    ) -> Result<Response<GetRoleResponse>, Status> {
        println!("Got a request: {:?}", request);

        let request = request.into_inner();

        let role = Role::find_by_id(request.id_role as i16).one(&self.db).await.map_err(|err| Status::internal(err.to_string()))?;

        let Some(r) = role else { return Ok(Response::new(GetRoleResponse{
            status: false,
            message: "notfound".to_lowercase().to_string(),
            ..Default::default()
        })) };

        let response = role::GetRoleResponse{
            status: true,
            message: "success".to_string(),
            role: Some(RoleItem{
                id: r.id as i32,
                libelle: r.libelle
            }),
        };

        Ok(Response::new(response))
    }

    async fn get_roles(
        &self,
        request: Request<GetRolesRequest>,
    ) -> Result<
        Response<GetRolesResponse>,
        Status> {
        println!("Got a request: {:?}", request);

        let roles = Role::find().all(&self.db).await.map_err(|err| Status::internal(err.to_string()))?;

        println!("{:?}", roles);

        let role_items: Vec<RoleItem> = match roles.len() > 0 {
            true => roles.iter().map(|r| RoleItem{
                id: r.id as i32,
                libelle: r.libelle.clone()
            }).collect(),
            false => return Ok(Response::new(GetRolesResponse{
                status: false,
                message: "nocontent".to_lowercase().to_string(),
                ..Default::default()
            }))
            
        };

        let response = role::GetRolesResponse{
            status: true,
            roles: role_items,
            ..Default::default()
        };

        Ok(Response::new(response))
    }

    async fn get_role_by_user_id(
        &self,
        request: Request<GetRoleByUserIdRequest>,
    ) -> Result<
        Response<GetRoleByUserIdResponse>,
        Status> {
        println!("Got a request: {:?}", request);

        let request = request.into_inner();

        if request.id == 0 {
            return Ok(Response::new(role::GetRoleByUserIdResponse{
                status: false,
                message: "badrequest".to_lowercase().to_string(),
                ..Default::default()
            }))
            
        }

        let roles = Role::find()
        .inner_join(User)
        .filter(user::Column::Id.eq(request.id))
        .all(&self.db)
        .await.map_err(|err| Status::internal(err.to_string()))?;

        let role_items: Vec<RoleItem> = match roles.len() > 0 {
            true => roles.iter().map(|r| RoleItem{
                id: r.id as i32,
                libelle: r.libelle.clone()
            }).collect(),
            false => return Ok(Response::new(GetRoleByUserIdResponse{
                status: false,
                message: "nocontent".to_lowercase().to_string(),
                ..Default::default()
            }))
            
        };

        let response = role::GetRoleByUserIdResponse{
            status: true,
            roles: role_items,
            ..Default::default()
        };

        Ok(Response::new(response))
    }

    async fn set_role_by_user_id(
        &self,
        request: Request<SetRoleByUserIdRequest>,
    ) -> Result<
        Response<SetRoleByUserIdResponse>,
        Status> {
        println!("Got a request: {:?}", request);

        let request = request.into_inner();

        let inserted = UsrAssoRol::insert(usr_asso_rol::ActiveModel{
            id_user: Set(*&request.user_id).to_owned(),
            id_role: Set(*&request.role_id as i16).to_owned()
        }).exec_with_returning(&self.db).await.map_err(|err| Status::internal(err.to_string()))?;

        match inserted.id_role == request.role_id as i16 && inserted.id_user == request.user_id{
            true => return Ok(Response::new(role::SetRoleByUserIdResponse{
                status: true,
                ..Default::default()
            })),
            false => return Ok(Response::new(role::SetRoleByUserIdResponse{
                status: false,
                message: "internalerror".to_lowercase().to_string(),
                ..Default::default()
            }))
        };
    }

    async fn unset_role_by_user_id(
        &self,
        request: Request<UnsetRoleByUserIdRequest>,
    ) -> Result<
        Response<UnsetRoleByUserIdResponse>,
        Status> {
        println!("Got a request: {:?}", request);

        let request = request.into_inner();

        let deleted = UsrAssoRol::delete(usr_asso_rol::ActiveModel{
            id_user: Set(*&request.user_id).to_owned(),
            id_role: Set(*&request.role_id as i16).to_owned()
        }).exec(&self.db).await.map_err(|err| Status::internal(err.to_string()))?;

         match deleted.rows_affected == 1{
            true => return Ok(Response::new(role::UnsetRoleByUserIdResponse{
                status: true,
                ..Default::default()
            })),
            false => return Ok(Response::new(role::UnsetRoleByUserIdResponse{
                status: false,
                message: "internalerror".to_lowercase().to_string(),
                ..Default::default()
            }))
         }
    }
}