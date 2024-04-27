use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
#[sea_orm(table_name = "role", schema_name = "public")]
pub struct Entity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    #[sea_orm(primary_key, column_name = "rol_Id" )]
    pub id: i16,
    #[sea_orm(column_name = "rol_Libelle" )]
    pub libelle: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveCustomColumn)]
pub enum Column {
    Id,
    Libelle,
}

impl IdenStatic for Column {
    fn as_str(&self) -> &str {
        match self {
            // Override column names
            Self::Id => "rol_Id",
            Self::Libelle => "rol_Libelle",
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i16;

    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Libelle => ColumnType::String(Some(50)).def(),
        }
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::usr_asso_rol::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::usr_asso_rol::Relation::Role.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}