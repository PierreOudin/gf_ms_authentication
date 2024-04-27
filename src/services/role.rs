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