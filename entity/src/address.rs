use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
#[sea_orm(table_name = "address", schema_name = "public")]
pub struct Entity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    #[sea_orm(primary_key, column_name = "adr_id" )]
    pub id: i32,
    #[sea_orm(column_name = "adr_address" )]
    pub address: String,
    #[sea_orm(column_name = "adr_zip" )]
    pub zip: String,
    #[sea_orm(column_name = "adr_city" )]
    pub city: String,
    #[sea_orm(column_name = "adr_additional" )]
    pub additional: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveCustomColumn)]
pub enum Column {
    Id,
    Address,
    Zip,
    City,
    Additional,
}

impl IdenStatic for Column {
    fn as_str(&self) -> &str {
        match self {
            // Override column names
            Self::Id => "adr_Id",
            Self::Address => "adr_Address",
            Self::Zip => "adr_Zip",
            Self::City => "adr_City",
            Self::Additional => "adr_Additional",
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;

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
            Self::Address => ColumnType::String(Some(50)).def(),
            Self::Zip => ColumnType::String(Some(50)).def(),
            Self::City => ColumnType::String(Some(50)).def(),
            Self::Additional => ColumnType::String(Some(50)).def().nullable(),
        }
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::usr_asso_adr::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::usr_asso_adr::Relation::Address.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}