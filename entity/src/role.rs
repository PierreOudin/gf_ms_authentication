use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String")]
pub enum Role {
    #[sea_orm(string_value = "Admin")]
    Admin,
    #[sea_orm(string_value = "Compta")]
    Compta,
    #[sea_orm(string_value = "Livreur")]
    Livreur,
    #[sea_orm(string_value = "Client")]
    Client
}

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
#[sea_orm(table_name = "role", schema_name = "public")]
pub struct Entity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
pub struct Model {
    #[sea_orm(primary_key, column_name = "rol_Id")]
    pub id: i16,
    #[sea_orm(column_name = "rol_Id")]
    pub libelle: Role,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::usr_asso_rol::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::usr_asso_rol::Relation::Role.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}