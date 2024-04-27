use entity::user;
use dotenvy;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::error::AuthError;

use super::claims::Claims;

pub(crate) fn create_token(user: user::Model) -> Result<String, AuthError> {
    let secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let claims = Claims::new(user);

    encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(secret.as_ref())).map_err(AuthError::InvalidToken)
}

pub(crate) fn validate_token(token: String) -> Result<Claims, AuthError> {
    let secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET must be set");

    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(AuthError::InvalidToken)
}