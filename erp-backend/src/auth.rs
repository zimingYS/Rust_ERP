use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
#[derive(Clone)]
pub struct Claims{
    pub sub:String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserClaims {
    pub sub: String,
    pub role_id: i32,
    pub exp: usize,
}

const JWT_SECRET:&[u8]=b"secret";
#[allow(dead_code)]
pub fn create_jwt(username:&str) ->Result<String,jsonwebtoken::errors::Error>{
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims{
        sub: username.to_owned(),
        exp: expiration,
    };

    encode(&Header::default(), &claims,&EncodingKey::from_secret(JWT_SECRET))
}

pub fn decode_jwt(token: &str) -> Result<Claims,jsonwebtoken::errors::Error>{
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS256),
    ).map(|data| data.claims)
}


pub fn create_user_jwt(username: &str, role_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = UserClaims {
        sub: username.to_owned(),
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