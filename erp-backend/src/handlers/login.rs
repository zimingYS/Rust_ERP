use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::auth::create_jwt;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct LoginRequest{
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse{
    pub code: String,
    pub token: String,
    pub msg: String,
}

pub async fn login(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) ->Result<Json<LoginResponse>, StatusCode>{
    if payload.username == "admin" && payload.password == "123456" {
        let token = create_jwt(&payload.username)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(LoginResponse { code: "200".to_string(), token, msg: "登录成功".to_string() }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}