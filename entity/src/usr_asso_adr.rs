use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
#[sea_orm(table_name = "usr_asso_adr", schema_name = "public")]
pub struct Entity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub id_user: i32,
    pub id_address: i32,
    pub is_default: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveCustomColumn)]
pub enum Column {
    IdUser,
    IdAddress,
    IsDefault,
}

impl IdenStatic for Column {
    fn as_str(&self) -> &str {
        match self {
            // Override column names
            Self::IdUser => "usr_Id",
            Self::IdAddress => "adr_Id",
            Self::IsDefault => "uaa_IsDefault"
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    #[sea_orm(column_name = "usr_Id" )]
    IdUser,
    #[sea_orm(column_name = "adr_Id" )]
    IdAddress
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (i32, i32);

    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    Address,
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::IdUser => ColumnType::Integer.def(),
            Self::IdAddress => ColumnType::Integer.def(),
            Self::IsDefault => ColumnType::Boolean.def().default_value(false),
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
            Self::Address => Entity::belongs_to(super::address::Entity)
                .from(Column::IdAddress)
                .to(super::address::Column::Id)
                .into()
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}