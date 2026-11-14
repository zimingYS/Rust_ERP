use crate::auth::{JWT_SECRET, UserClaims};
use crate::constants::ROLE_ADMIN;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

pub async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));

    let token = auth_header.ok_or(StatusCode::UNAUTHORIZED)?;

    use jsonwebtoken::{DecodingKey, Validation, decode};

    let decoded = decode::<UserClaims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let mut req = req;
    req.extensions_mut().insert(decoded.claims);

    Ok(next.run(req).await)
}

pub async fn require_admin_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let claims = req
        .extensions()
        .get::<UserClaims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if claims.role_id != ROLE_ADMIN {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(req).await)
}
