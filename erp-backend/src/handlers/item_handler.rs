use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set};
use serde::Deserialize;
use crate::entities::items::{ActiveModel, Column, Entity, Model};
use crate::handlers::error::AppError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateItemRequest{
    name:String,
    sku:String,
    stock:i32,
}

#[derive(Deserialize)]
pub struct UpdateItemRequest{
    name: String,
    sku: String,
    stock: i32,
}

pub async fn get_items(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Model>>, AppError> {
    let items = Entity::find()
        .order_by_asc(Column::Id)
        .all(&state.db)
        .await?;

    Ok(Json(items))
}

// 新增物料
pub async fn create_item(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateItemRequest>,
) -> Result<(StatusCode, Json<Model>), AppError> {
    let new_item = ActiveModel{
        name: Set(payload.name),
        sku: Set(payload.sku),
        stock: Set(payload.stock),
        ..Default::default()
    };

    let inserted_item = new_item.insert(&state.db).await?;

    Ok((StatusCode::CREATED, Json(inserted_item)))
}

// 删除物料
pub async fn delete_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
)-> Result<StatusCode, AppError> {
    // 使用 SeaORM 内置删除
    let res = Entity::delete_by_id(id)
        .exec(&state.db)
        .await?;

    if res.rows_affected > 0 {
        Ok(StatusCode::NO_CONTENT) // 204 成功但无内容返回
    } else {
        Err(AppError::NotFound(format!("找不到 ID 为 {} 的物料，无法删除", id))) // 404 ID 不存在
    }
}

// 更新物料
pub async fn update_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateItemRequest>,
) -> Result<StatusCode, AppError> {
    let item = Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("ID 为 {} 的物料不存在", id)))?;

    let mut item: ActiveModel = item.into();
    item.name = Set(payload.name);
    item.sku = Set(payload.sku);
    item.stock = Set(payload.stock);

    item.update(&state.db).await?;
    Ok(StatusCode::OK)
}