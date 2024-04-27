
use serde::{Deserialize, Serialize};
use entity::user;
use chrono::Utc;

const JWT_ISSUER: &str = "authentication";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    // issuer
    pub iss: String,
    // subject
    pub sub: String,
    // issued at
    pub iat: i64,

    pub fistname: String,

    pub lastname: String,
    // user email
    pub email: String,
}
 
impl Claims {
    pub fn new(user: user::Model) -> Self {
        let iat = Utc::now();

        Claims {
            iss: JWT_ISSUER.to_string(),
            sub: user.id.to_string(),
            iat: iat.timestamp(),
            fistname: user.firstname,
            lastname: user.lastname,
            email: user.email,
        }
    }
}