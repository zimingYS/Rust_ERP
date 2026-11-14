use crate::constants::TOKEN_EXPIRY_SECONDS;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) const JWT_SECRET: &str = "your-secret-key";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserClaims {
    pub user_id: i32,
    pub username: String,
    pub role_id: i32,
    pub exp: usize,
}

pub fn create_token(
    user_id: i32,
    username: String,
    role_id: i32,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + TOKEN_EXPIRY_SECONDS;

    let claims = UserClaims {
        user_id,
        username,
        role_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
}
