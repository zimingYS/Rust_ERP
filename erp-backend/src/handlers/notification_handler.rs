/*
    消息通知接口
*/
use crate::auth::UserClaims;
use crate::entities::notification;
use crate::handlers::error::AppError;
use crate::state::AppState;
use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::http::StatusCode;
use chrono::Utc;
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct NotificationQueryParams {
    pub notification_type: Option<String>,
    pub is_read: Option<bool>,
    pub priority: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateNotificationRequest {
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub notification_type: String,
    pub priority: Option<String>,
    pub related_id: Option<i32>,
    pub related_type: Option<String>,
}

#[derive(Serialize)]
pub struct NotificationStats {
    pub total_count: u64,
    pub unread_count: u64,
    pub high_priority_count: u64,
}

pub async fn get_user_notifications(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<UserClaims>,
    Query(params): Query<NotificationQueryParams>,
) -> Result<Json<Vec<notification::Model>>, AppError> {
    let mut query =
        notification::Entity::find().filter(notification::Column::UserId.eq(claims.user_id));

    if let Some(notification_type) = &params.notification_type {
        query = query.filter(notification::Column::NotificationType.eq(notification_type));
    }

    if let Some(is_read) = params.is_read {
        query = query.filter(notification::Column::IsRead.eq(is_read));
    }

    if let Some(priority) = &params.priority {
        query = query.filter(notification::Column::Priority.eq(priority));
    }

    let notifications = query
        .order_by_desc(notification::Column::CreatedAt)
        .all(&state.db)
        .await?;

    Ok(Json(notifications))
}

pub async fn get_notification_stats(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<UserClaims>,
) -> Result<Json<NotificationStats>, AppError> {
    let all_notifications = notification::Entity::find()
        .filter(notification::Column::UserId.eq(claims.user_id))
        .all(&state.db)
        .await?;

    let total_count = all_notifications.len() as u64;
    let unread_count = all_notifications.iter().filter(|n| !n.is_read).count() as u64;
    let high_priority_count = all_notifications
        .iter()
        .filter(|n| n.priority.as_deref() == Some("高"))
        .count() as u64;

    Ok(Json(NotificationStats {
        total_count,
        unread_count,
        high_priority_count,
    }))
}

pub async fn mark_as_read(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<UserClaims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let notification_model = notification::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("消息 {} 不存在", id)))?;

    if notification_model.user_id != claims.user_id {
        return Err(AppError::Forbidden("无权操作此消息".to_string()));
    }

    let mut notification: notification::ActiveModel = notification_model.into();
    notification.is_read = Set(true);
    notification.read_at = Set(Some(Utc::now()));

    notification.update(&state.db).await?;

    tracing::info!("标记消息为已读: ID={}, 用户ID={}", id, claims.user_id);
    Ok(StatusCode::OK)
}

pub async fn mark_all_as_read(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<UserClaims>,
) -> Result<StatusCode, AppError> {
    notification::Entity::update_many()
        .col_expr(notification::Column::IsRead, Expr::value(true))
        .col_expr(notification::Column::ReadAt, Expr::value(Utc::now()))
        .filter(notification::Column::UserId.eq(claims.user_id))
        .filter(notification::Column::IsRead.eq(false))
        .exec(&state.db)
        .await?;

    tracing::info!("标记所有消息为已读: 用户ID={}", claims.user_id);
    Ok(StatusCode::OK)
}

pub async fn delete_notification(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<UserClaims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let notification_model = notification::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("消息 {} 不存在", id)))?;

    if notification_model.user_id != claims.user_id {
        return Err(AppError::Forbidden("无权删除此消息".to_string()));
    }

    let res = notification::Entity::delete_by_id(id)
        .exec(&state.db)
        .await?;

    if res.rows_affected > 0 {
        tracing::info!("删除消息成功: ID={}, 用户ID={}", id, claims.user_id);
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound(format!("消息 {} 不存在", id)))
    }
}

pub async fn create_notification(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateNotificationRequest>,
) -> Result<(StatusCode, Json<notification::Model>), AppError> {
    if payload.title.is_empty() || payload.content.is_empty() {
        return Err(AppError::BadRequest("标题和内容不能为空".to_string()));
    }

    let now = Utc::now();

    let new_notification = notification::ActiveModel {
        user_id: Set(payload.user_id),
        title: Set(payload.title),
        content: Set(payload.content),
        notification_type: Set(payload.notification_type),
        is_read: Set(false),
        priority: Set(payload.priority),
        related_id: Set(payload.related_id),
        related_type: Set(payload.related_type),
        created_at: Set(now),
        read_at: Set(None),
        ..Default::default()
    };

    let inserted_notification = new_notification.insert(&state.db).await?;

    tracing::info!(
        "创建消息通知: 用户ID={}, 类型={}",
        payload.user_id,
        inserted_notification.notification_type
    );
    Ok((StatusCode::CREATED, Json(inserted_notification)))
}

pub async fn get_unread_count(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<UserClaims>,
) -> Result<Json<u64>, AppError> {
    let count = notification::Entity::find()
        .filter(notification::Column::UserId.eq(claims.user_id))
        .filter(notification::Column::IsRead.eq(false))
        .count(&state.db)
        .await?;

    Ok(Json(count))
}
