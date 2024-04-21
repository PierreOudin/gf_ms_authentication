pub mod role;
pub mod user;
pub mod usr_asso_rol;
pub mod address;
pub mod usr_asso_adr;

pub use role::Entity as Role;
pub use user::Entity as User;
pub use usr_asso_rol::Entity as UsrAssoRol;
pub use address::Entity as Address;
pub use usr_asso_adr::Entity as UsrAssoAdr;

pub use sea_orm;