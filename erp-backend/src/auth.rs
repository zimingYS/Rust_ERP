use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserClaims {
    pub sub: String,
    pub user_id: i32,
    pub role_id: i32,
    pub exp: usize,
}

const JWT_SECRET:&[u8]=b"secret";
pub fn create_user_jwt(username: &str, user_id: i32, role_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = UserClaims {
        sub: username.to_owned(),
        user_id,
        role_id,
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))
}

pub fn decode_user_jwt(token: &str) -> Result<UserClaims, jsonwebtoken::errors::Error> {
    decode::<UserClaims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS256),
    ).map(|data| data.claims)
}