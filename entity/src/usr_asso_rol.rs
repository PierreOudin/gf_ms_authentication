use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
#[sea_orm(table_name = "usr_asso_rol", schema_name = "public")]
pub struct Entity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id_role: i16,
    pub id_user: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveCustomColumn)]
pub enum Column {
    IdRole,
    IdUser,
}

impl IdenStatic for Column {
    fn as_str(&self) -> &str {
        match self {
            // Override column names
            Self::IdRole => "rol_Id",
            Self::IdUser => "usr_Id",
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    #[sea_orm(column_name = "rol_Id" )]
    IdRole,
    #[sea_orm(column_name = "usr_Id" )]
    IdUser,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (i16, i32);

    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    Role,
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::IdRole => ColumnType::Integer.def(),
            Self::IdUser => ColumnType::Integer.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::IdUser)
                .to(super::user::Column::Id)
                .into(),
            Self::Role => Entity::belongs_to(super::role::Entity)
                .from(Column::IdRole)
                .to(super::role::Column::Id)
                .into()
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}