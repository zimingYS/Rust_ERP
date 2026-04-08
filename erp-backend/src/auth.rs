use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
#[derive(Clone)]
pub struct Claims{
    pub sub:String,
    pub exp: usize,
}

const JWT_SECRET:&[u8]=b"secret";
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