use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
#[sea_orm(table_name = "user", schema_name = "public")]
pub struct Entity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    #[sea_orm(primary_key, column_name = "usr_Id" )]
    pub id: i32,
    #[sea_orm(column_name = "usr_FirstName" )]
    pub firstname: String,
    #[sea_orm(column_name = "usr_LastName" )]
    pub lastname: String,
    #[sea_orm(column_name = "usr_Email" )]
    pub email: String,
    #[sea_orm(column_name = "usr_Phone" )]
    pub phone: String,
    #[sea_orm(column_name = "usr_Password" )]
    pub password: String,
    #[sea_orm(column_name = "res_Id" )]
    pub id_restaurant: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveCustomColumn)]
pub enum Column {
    Id,
    Firstname,
    Lastname,
    Email,
    Phone,
    Password,
    IdRestaurant,
}

impl IdenStatic for Column {
    fn as_str(&self) -> &str {
        match self {
            // Override column names
            Self::Id => "usr_Id",
            Self::Firstname => "usr_FirstName",
            Self::Lastname => "usr_LastName",
            Self::Email => "usr_Email",
            Self::Phone => "usr_Phone",
            Self::Password => "usr_Password",
            Self::IdRestaurant => "res_Id",
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
            Self::Firstname => ColumnType::String(Some(50)).def(),
            Self::Lastname => ColumnType::String(Some(50)).def(),
            Self::Email => ColumnType::String(Some(50)).def(),
            Self::Phone => ColumnType::String(Some(50)).def(),
            Self::Password => ColumnType::String(Some(50)).def(),
            Self::IdRestaurant => ColumnType::Integer.def().nullable(),
        }
    }
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        super::usr_asso_rol::Relation::Role.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::usr_asso_rol::Relation::User.def().rev())
    }
}

impl Related<super::address::Entity> for Entity {
    fn to() -> RelationDef {
        super::usr_asso_adr::Relation::Address.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::usr_asso_adr::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}