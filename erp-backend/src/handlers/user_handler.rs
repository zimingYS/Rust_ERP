use crate::auth;
use crate::constants::ROLE_USER;
use crate::entities::user;
use crate::handlers::error::AppError;
use crate::state::AppState;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use jsonwebtoken::signature::rand_core::OsRng;
use password_hash::SaltString;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    let exists = user::Entity::find()
        .filter(user::Column::Username.eq(&payload.username))
        .one(&state.db)
        .await?
        .is_some();

    if exists {
        return Err(AppError::BadRequest("用户名已存在".to_string()));
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| AppError::InternalError(format!("密码加密失败: {}", e)))?
        .to_string();

    tracing::info!("尝试注册用户: {}", payload.username);

    let new_user = user::ActiveModel {
        username: Set(payload.username),
        password_hash: Set(password_hash),
        role_id: Set(ROLE_USER),
        ..Default::default()
    };

    let res = new_user.insert(&state.db).await?;

    tracing::info!("用户注册成功! ID: {}, 角色: 普通用户", res.id);

    Ok((axum::http::StatusCode::OK, Json(json!({"msg": "注册成功"}))))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(&payload.username))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Unauthorized)?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|e| AppError::InternalError(format!("密码格式错误: {}", e)))?;

    let valid = Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !valid {
        return Err(AppError::Unauthorized);
    }

    let token = auth::create_token(user.id, user.username.clone(), user.role_id)
        .map_err(|e| AppError::InternalError(format!("令牌生成失败: {}", e)))?;

    Ok((axum::http::StatusCode::OK, Json(json!({"token": token}))))
}
