use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::{StatusCode, header},
};
use crate::auth::decode_jwt;

pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    if let Some(auth_header) = auth_header {
        // 格式通常为 "Bearer <token>"
        let token = auth_header.trim_start_matches("Bearer ").trim();

        if let Ok(claims) = decode_jwt(token) {
            // 将用户信息插入 Request 扩展中，后续 Handler 可以获取
            req.extensions_mut().insert(claims);
            return Ok(next.run(req).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}