/*
    审计日志接口
*/
use crate::entities::audit_log;
use crate::handlers::error::AppError;
use crate::state::AppState;
use axum::Json;
use axum::extract::{Query, State};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct AuditLogQueryParams {
    pub user_id: Option<i32>,
    pub username: Option<String>,
    pub module: Option<String>,
    pub action: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateAuditLogRequest {
    pub user_id: i32,
    pub username: String,
    pub action: String,
    pub module: String,
    pub method: String,
    pub path: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub request_data: Option<String>,
    pub response_status: i32,
    pub description: Option<String>,
}

pub async fn get_audit_logs(
    State(state): State<Arc<AppState>>,
    Query(params): Query<AuditLogQueryParams>,
) -> Result<Json<Vec<audit_log::Model>>, AppError> {
    let mut query = audit_log::Entity::find();

    if let Some(user_id) = params.user_id {
        query = query.filter(audit_log::Column::UserId.eq(user_id));
    }

    if let Some(username) = &params.username {
        query = query.filter(audit_log::Column::Username.contains(username));
    }

    if let Some(module) = &params.module {
        query = query.filter(audit_log::Column::Module.eq(module));
    }

    if let Some(action) = &params.action {
        query = query.filter(audit_log::Column::Action.contains(action));
    }

    if let Some(start_date) = &params.start_date {
        query = query.filter(audit_log::Column::CreatedAt.gte(start_date));
    }

    if let Some(end_date) = &params.end_date {
        query = query.filter(audit_log::Column::CreatedAt.lte(end_date));
    }

    let logs = query
        .order_by_desc(audit_log::Column::CreatedAt)
        .all(&state.db)
        .await?;

    Ok(Json(logs))
}

pub async fn create_audit_log(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateAuditLogRequest>,
) -> Result<Json<audit_log::Model>, AppError> {
    use chrono::Local;
    use sea_orm::{ActiveModelTrait, Set};

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let new_log = audit_log::ActiveModel {
        user_id: Set(payload.user_id),
        username: Set(payload.username),
        action: Set(payload.action),
        module: Set(payload.module),
        method: Set(payload.method),
        path: Set(payload.path),
        ip_address: Set(payload.ip_address),
        user_agent: Set(payload.user_agent),
        request_data: Set(payload.request_data),
        response_status: Set(payload.response_status),
        description: Set(payload.description),
        created_at: Set(now),
        ..Default::default()
    };

    let inserted = new_log.insert(&state.db).await?;
    Ok(Json(inserted))
}

pub async fn get_user_activity_summary(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let all_logs = audit_log::Entity::find().all(&state.db).await?;

    let mut user_activity: std::collections::HashMap<String, u64> =
        std::collections::HashMap::new();
    let mut module_activity: std::collections::HashMap<String, u64> =
        std::collections::HashMap::new();

    for log in &all_logs {
        *user_activity.entry(log.username.clone()).or_insert(0) += 1;
        *module_activity.entry(log.module.clone()).or_insert(0) += 1;
    }

    Ok(Json(serde_json::json!({
        "total_logs": all_logs.len(),
        "user_activity": user_activity,
        "module_activity": module_activity
    })))
}
