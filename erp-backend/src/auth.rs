use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) const JWT_SECRET: &str = "your-secret-key";
const TOKEN_EXPIRATION_SECONDS: usize = 86400;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserClaims {
    pub user_id: i32,
    pub username: String,
    pub role_id: i32,
    pub exp: usize,
}

pub fn create_token(user_id: i32, username: String, role_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize + TOKEN_EXPIRATION_SECONDS;

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
