use crate::entities::user;
use crate::state::AppState;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use jsonwebtoken::signature::rand_core::OsRng;
use jsonwebtoken::{encode, EncodingKey, Header};
use password_hash::SaltString;
use sea_orm::{ActiveModelTrait, EntityTrait, Set, QueryFilter, ColumnTrait, Order, QueryOrder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use crate::auth;

#[derive(Deserialize)]
pub struct LoginRequest{
    pub username: String,
    pub password: String,
}

// #[derive(Serialize)]
// pub struct LoginResponse{
//     pub code: String,
//     pub token: String,
//     pub msg: String,
// }

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

// 注册
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>
) -> impl IntoResponse {
    // 检查用户名是否存在
    let exists = match user::Entity::find()
        .filter(user::Column::Username.eq(&payload.username))
        .one(&state.db)
        .await {
        Ok(result) => result.is_some(),
        Err(e) => {
            tracing::error!("查询用户失败: {:?}", e);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("查询失败: {}", e)}))
            ).into_response();
        }
    };

    if exists {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({"error": "用户名已存在"}))
        ).into_response();
    }

    // 密码加密
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = match Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(e) => {
            tracing::error!("密码加密失败: {:?}", e);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("密码加密失败: {}", e)}))
            ).into_response();
        }
    };

    tracing::info!("尝试注册用户: {}", payload.username);

    let default_role_id = 1;

    tracing::info!("为用户 {} 分配默认角色ID: {} (普通用户)", payload.username, default_role_id);


    // 插入数据库
    let new_user = user::ActiveModel {
        username: Set(payload.username),
        password_hash: Set(password_hash),
        role_id: Set(default_role_id),
        ..Default::default()
    };

    let res = new_user.insert(&state.db).await;
    match res {
        Ok(user) => {
            tracing::info!("用户注册成功! ID: {}", user.id);
            (
                axum::http::StatusCode::OK,
                Json(json!({"msg": "注册成功"}))
            ).into_response()
        },
        Err(e) => {
            tracing::error!("用户注册失败! 错误详情: {:?}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("注册失败: {}", e)}))
            ).into_response()
        },
    }
}

// 登录
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>
) -> impl IntoResponse {
    // 查找用户
    let user = match user::Entity::find()
        .filter(user::Column::Username.eq(&payload.username))
        .one(&state.db)
        .await {
        Ok(result) => result,
        Err(e) => {
            tracing::error!("数据库查询失败: {:?}", e);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "服务器内部错误"}))
            ).into_response();
        }
    };

    let user = match user {
        Some(u) => u,
        None => {
            tracing::info!("用户不存在: {}", payload.username);
            return (
                axum::http::StatusCode::UNAUTHORIZED,
                Json(json!({"error": "用户不存在"}))
            ).into_response()
        },
    };

    // 校验密码
    let parsed_hash = match PasswordHash::new(&user.password_hash) {
        Ok(hash) => hash,
        Err(e) => {
            tracing::error!("密码格式错误: {:?}", e);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "密码格式错误"}))
            ).into_response();
        }
    };

    let valid = Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !valid {
        tracing::info!("密码错误: {}", payload.username);
        return (
            axum::http::StatusCode::UNAUTHORIZED,
            Json(json!({"error": "密码错误"}))
        ).into_response();
    }

    match auth::create_user_jwt(&user.username, user.role_id) {
        Ok(token) => (
            axum::http::StatusCode::OK,
            Json(json!({"token": token}))
        ).into_response(),
        Err(e) => {
            tracing::error!("令牌生成失败: {:?}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "令牌生成失败"}))
            ).into_response()
        },
    }
}