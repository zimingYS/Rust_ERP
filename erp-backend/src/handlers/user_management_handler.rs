/*
    用户管理接口
*/
use std::sync::Arc;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait, Set, QueryOrder};
use serde::{Deserialize, Serialize};
use crate::entities::{user, role};
use crate::handlers::error::AppError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct UserQueryParams {
    pub username: Option<String>,
    pub role_id: Option<i32>,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub role_id: i32,
    pub role_name: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub role_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub role_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

pub async fn get_users(
    State(state): State<Arc<AppState>>,
    Query(params): Query<UserQueryParams>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let mut query = user::Entity::find();

    if let Some(username) = &params.username {
        query = query.filter(user::Column::Username.contains(username));
    }

    if let Some(role_id) = params.role_id {
        query = query.filter(user::Column::RoleId.eq(role_id));
    }

    let users = query
        .order_by_asc(user::Column::Id)
        .all(&state.db)
        .await?;

    let mut user_responses = Vec::new();
    for user_model in users {
        let role_entity = role::Entity::find_by_id(user_model.role_id)
            .one(&state.db)
            .await?;

        let role_name = role_entity.map(|r| r.name).unwrap_or_else(|| "未知".to_string());

        user_responses.push(UserResponse {
            id: user_model.id,
            username: user_model.username,
            role_id: user_model.role_id,
            role_name,
        });
    }

    Ok(Json(user_responses))
}

pub async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<UserResponse>, AppError> {
    let user_model = user::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("用户 {} 不存在", id)))?;

    let role_entity = role::Entity::find_by_id(user_model.role_id)
        .one(&state.db)
        .await?;

    let role_name = role_entity.map(|r| r.name).unwrap_or_else(|| "未知".to_string());

    Ok(Json(UserResponse {
        id: user_model.id,
        username: user_model.username,
        role_id: user_model.role_id,
        role_name,
    }))
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    // 验证用户名是否已存在
    let existing_user = user::Entity::find()
        .filter(user::Column::Username.eq(&payload.username))
        .one(&state.db)
        .await?;

    if existing_user.is_some() {
        return Err(AppError::BadRequest("用户名已存在".to_string()));
    }

    // 验证角色是否存在
    let role_exists = role::Entity::find_by_id(payload.role_id)
        .one(&state.db)
        .await?;

    if role_exists.is_none() {
        return Err(AppError::BadRequest("指定的角色不存在".to_string()));
    }

    // 密码加密
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| AppError::InternalError(format!("密码加密失败: {}", e)))?
        .to_string();

    let new_user = user::ActiveModel {
        username: Set(payload.username.clone()),
        password_hash: Set(password_hash),
        role_id: Set(payload.role_id),
        ..Default::default()
    };

    let inserted = new_user.insert(&state.db).await?;

    let role_name = role_exists.unwrap().name;

    tracing::info!("创建用户成功: {}", payload.username);

    Ok((StatusCode::CREATED, Json(UserResponse {
        id: inserted.id,
        username: inserted.username,
        role_id: inserted.role_id,
        role_name,
    })))
}

pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user_model = user::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("用户 {} 不存在", id)))?;

    let mut user_active: user::ActiveModel = user_model.clone().into();

    if let Some(role_id) = payload.role_id {
        // 验证角色是否存在
        let role_exists = role::Entity::find_by_id(role_id)
            .one(&state.db)
            .await?;

        if role_exists.is_none() {
            return Err(AppError::BadRequest("指定的角色不存在".to_string()));
        }

        user_active.role_id = Set(role_id);
    }

    let updated = user_active.update(&state.db).await?;

    let role_entity = role::Entity::find_by_id(updated.role_id)
        .one(&state.db)
        .await?;

    let role_name = role_entity.map(|r| r.name).unwrap_or_else(|| "未知".to_string());

    tracing::info!("更新用户信息: ID={}", id);

    Ok(Json(UserResponse {
        id: updated.id,
        username: updated.username,
        role_id: updated.role_id,
        role_name,
    }))
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let user_model = user::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("用户 {} 不存在", id)))?;

    // 不允许删除第一个用户（超级管理员）
    if user_model.id == 1 {
        return Err(AppError::BadRequest("不能删除超级管理员账户".to_string()));
    }

    user::Entity::delete_by_id(id)
        .exec(&state.db)
        .await?;

    tracing::info!("删除用户: {}", user_model.username);

    Ok(StatusCode::NO_CONTENT)
}

pub async fn change_password(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<StatusCode, AppError> {
    let user_model = user::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("用户 {} 不存在", id)))?;

    // 验证旧密码
    use argon2::password_hash::{PasswordHash, PasswordVerifier};
    use argon2::Argon2;

    let parsed_hash = PasswordHash::new(&user_model.password_hash)
        .map_err(|_| AppError::InternalError("密码格式错误".to_string()))?;

    let result = Argon2::default().verify_password(
        payload.old_password.as_bytes(),
        &parsed_hash
    );

    if result.is_err() {
        return Err(AppError::BadRequest("旧密码不正确".to_string()));
    }

    // 生成新密码哈希
    use argon2::password_hash::{rand_core::OsRng, SaltString, PasswordHasher};

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let new_password_hash = argon2
        .hash_password(payload.new_password.as_bytes(), &salt)
        .map_err(|e| AppError::InternalError(format!("密码加密失败: {}", e)))?
        .to_string();

    let mut user_active: user::ActiveModel = user_model.into();
    user_active.password_hash = Set(new_password_hash);
    user_active.update(&state.db).await?;

    tracing::info!("用户修改密码: ID={}", id);

    Ok(StatusCode::OK)
}

pub async fn reset_password(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let user_model = user::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("用户 {} 不存在", id)))?;

    // 重置为默认密码 123456
    use argon2::password_hash::{rand_core::OsRng, SaltString, PasswordHasher};
    use argon2::Argon2;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let default_password = "123456";
    let password_hash = argon2
        .hash_password(default_password.as_bytes(), &salt)
        .map_err(|e| AppError::InternalError(format!("密码加密失败: {}", e)))?
        .to_string();

    let mut user_active: user::ActiveModel = user_model.into();
    user_active.password_hash = Set(password_hash);
    user_active.update(&state.db).await?;

    tracing::info!("管理员重置用户密码: ID={}", id);

    Ok(StatusCode::OK)
}

pub async fn get_roles(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<role::Model>>, AppError> {
    let roles = role::Entity::find()
        .order_by_asc(role::Column::Id)
        .all(&state.db)
        .await?;

    Ok(Json(roles))
}
