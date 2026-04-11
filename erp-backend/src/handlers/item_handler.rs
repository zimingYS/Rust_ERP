use std::sync::Arc;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set, QueryFilter, ColumnTrait, PaginatorTrait};
use serde::{Deserialize, Serialize};
use crate::entities::items::{ActiveModel, Column, Entity, Model};
use crate::handlers::error::AppError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub sku: String,
    pub stock: i32,
    pub description: Option<String>,
    pub unit: Option<String>,
    pub min_stock: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateItemRequest {
    pub name: String,
    pub sku: String,
    pub stock: i32,
    pub description: Option<String>,
    pub unit: Option<String>,
    pub min_stock: Option<i32>,
}

#[derive(Deserialize)]
pub struct ItemStockAdjustmentRequest {
    pub adjustment: i32,
    pub reason: Option<String>,
}

#[derive(Deserialize)]
pub struct ItemQueryParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
    pub sku: Option<String>,
}

#[derive(Serialize)]
pub struct PaginatedItems {
    pub items: Vec<Model>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

#[derive(Serialize)]
pub struct ItemResponse {
    pub code: u16,
    pub message: String,
    pub data: Option<Model>,
}

#[derive(Serialize)]
pub struct ItemsListResponse {
    pub code: u16,
    pub message: String,
    pub data: PaginatedItems,
}

pub async fn get_items(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ItemQueryParams>,
) -> Result<Json<ItemsListResponse>, AppError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10).min(100);

    let mut query = Entity::find();

    if let Some(keyword) = &params.keyword {
        query = query.filter(Column::Name.contains(keyword));
    }

    if let Some(sku) = &params.sku {
        query = query.filter(Column::Sku.eq(sku));
    }

    let total = query.clone().count(&state.db).await?;
    let total_pages = (total as f64 / page_size as f64).ceil() as u64;

    let items = query
        .order_by_asc(Column::Id)
        .paginate(&state.db, page_size)
        .fetch_page(page - 1)
        .await?;

    Ok(Json(ItemsListResponse {
        code: 200,
        message: "success".to_string(),
        data: PaginatedItems {
            items,
            total,
            page,
            page_size,
            total_pages,
        },
    }))
}

pub async fn get_item_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<ItemResponse>, AppError> {
    let item = Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("ID 为 {} 的物料不存在", id)))?;

    Ok(Json(ItemResponse {
        code: 200,
        message: "success".to_string(),
        data: Some(item),
    }))
}

pub async fn create_item(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateItemRequest>,
) -> Result<(StatusCode, Json<ItemResponse>), AppError> {
    let exists = Entity::find()
        .filter(Column::Sku.eq(&payload.sku))
        .one(&state.db)
        .await?;

    if exists.is_some() {
        return Err(AppError::BadRequest(format!("SKU {} 已存在", payload.sku)));
    }

    if payload.stock < 0 {
        return Err(AppError::BadRequest("库存不能为负数".to_string()));
    }

    let new_item = ActiveModel {
        name: Set(payload.name),
        sku: Set(payload.sku),
        stock: Set(payload.stock),
        description: Set(payload.description),
        unit: Set(payload.unit),
        min_stock: Set(payload.min_stock),
        ..Default::default()
    };

    let inserted_item = new_item.insert(&state.db).await?;

    Ok((StatusCode::CREATED, Json(ItemResponse {
        code: 201,
        message: "创建成功".to_string(),
        data: Some(inserted_item),
    })))
}

pub async fn update_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateItemRequest>,
) -> Result<Json<ItemResponse>, AppError> {
    let item = Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("ID 为 {} 的物料不存在", id)))?;

    let existing_sku = Entity::find()
        .filter(Column::Sku.eq(&payload.sku))
        .filter(Column::Id.ne(id))
        .one(&state.db)
        .await?;

    if existing_sku.is_some() {
        return Err(AppError::BadRequest(format!("SKU {} 已被其他物料使用", payload.sku)));
    }

    if payload.stock < 0 {
        return Err(AppError::BadRequest("库存不能为负数".to_string()));
    }

    let mut item: ActiveModel = item.into();
    item.name = Set(payload.name);
    item.sku = Set(payload.sku);
    item.stock = Set(payload.stock);
    item.description = Set(payload.description);
    item.unit = Set(payload.unit);
    item.min_stock = Set(payload.min_stock);

    let updated_item = item.update(&state.db).await?;

    Ok(Json(ItemResponse {
        code: 200,
        message: "更新成功".to_string(),
        data: Some(updated_item),
    }))
}

pub async fn delete_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let res = Entity::delete_by_id(id)
        .exec(&state.db)
        .await?;

    if res.rows_affected > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound(format!("找不到 ID 为 {} 的物料", id)))
    }
}

pub async fn adjust_stock(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<ItemStockAdjustmentRequest>,
) -> Result<Json<ItemResponse>, AppError> {
    let item = Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("ID 为 {} 的物料不存在", id)))?;

    let new_stock = item.stock + payload.adjustment;

    if new_stock < 0 {
        return Err(AppError::BadRequest(format!(
            "库存不足，当前库存: {}, 调整数量: {}",
            item.stock, payload.adjustment
        )));
    }

    let mut item: ActiveModel = item.into();
    item.stock = Set(new_stock);

    let updated_item = item.update(&state.db).await?;

    tracing::info!(
        "库存调整 - 物料ID: {}, 调整量: {}, 原因: {:?}, 新库存: {}",
        id, payload.adjustment, payload.reason, new_stock
    );

    Ok(Json(ItemResponse {
        code: 200,
        message: "库存调整成功".to_string(),
        data: Some(updated_item),
    }))
}

pub async fn batch_delete_items(
    State(state): State<Arc<AppState>>,
    Json(ids): Json<Vec<i32>>,
) -> Result<Json<serde_json::Value>, AppError> {
    if ids.is_empty() {
        return Err(AppError::BadRequest("请提供要删除的物料ID列表".to_string()));
    }

    let result = Entity::delete_many()
        .filter(Column::Id.is_in(ids.clone()))
        .exec(&state.db)
        .await?;

    tracing::info!("批量删除物料: {:?}, 影响行数: {}", ids, result.rows_affected);

    Ok(Json(serde_json::json!({
        "code": 200,
        "message": format!("成功删除 {} 个物料", result.rows_affected),
        "deleted_count": result.rows_affected
    })))
}