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