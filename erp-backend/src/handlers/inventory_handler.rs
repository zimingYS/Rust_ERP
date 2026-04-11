/*
    库存管理接口
*/
use std::sync::Arc;
use axum::extract::{Path, Query, State, Extension};
use axum::http::StatusCode;
use axum::Json;
use chrono::Local;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set, QueryFilter, ColumnTrait};
use serde::{Deserialize, Serialize};
use crate::entities::{items, inventory_adjustment};
use crate::handlers::error::AppError;
use crate::state::AppState;
use crate::auth::UserClaims;
use crate::constants::{
    ADJUSTMENT_TYPE_INBOUND,
    ADJUSTMENT_TYPE_OUTBOUND,
    ADJUSTMENT_TYPE_COUNTING,
    ADJUSTMENT_TYPE_LOSS,
    ADJUSTMENT_TYPE_RETURN,
};

#[derive(Deserialize)]
pub struct InventoryQueryParams {
    pub keyword: Option<String>,
    pub sku: Option<String>,
    pub low_stock_only: Option<bool>,
}

#[derive(Deserialize)]
pub struct StockAdjustmentRequest {
    pub adjustment_type: String,
    pub quantity: i32,
    pub reason: Option<String>,
}

#[derive(Serialize)]
pub struct InventoryItem {
    pub id: i32,
    pub name: String,
    pub sku: String,
    pub stock: i32,
    pub unit: Option<String>,
    pub min_stock: Option<i32>,
    pub is_low_stock: bool,
}

#[derive(Serialize)]
pub struct InventoryStats {
    pub total_items: u64,
    pub low_stock_items: u64,
    pub out_of_stock_items: u64,
    pub total_stock_value: u64,
}

fn build_inventory_item(item: &items::Model) -> InventoryItem {
    let min_stock = item.min_stock.unwrap_or(0);
    let is_low_stock = item.stock <= min_stock && item.stock > 0;

    InventoryItem {
        id: item.id,
        name: item.name.clone(),
        sku: item.sku.clone(),
        stock: item.stock,
        unit: item.unit.clone(),
        min_stock: item.min_stock,
        is_low_stock,
    }
}

pub async fn get_inventory_list(
    State(state): State<Arc<AppState>>,
    Query(params): Query<InventoryQueryParams>,
) -> Result<Json<Vec<InventoryItem>>, AppError> {
    let mut query = items::Entity::find();

    if let Some(keyword) = &params.keyword {
        query = query.filter(items::Column::Name.contains(keyword));
    }

    if let Some(sku) = &params.sku {
        query = query.filter(items::Column::Sku.eq(sku));
    }

    let all_items = query.all(&state.db).await?;

    let inventory_items: Vec<InventoryItem> = all_items.iter()
        .map(build_inventory_item)
        .collect();

    let filtered_items: Vec<InventoryItem> = if params.low_stock_only == Some(true) {
        inventory_items.into_iter().filter(|item| item.is_low_stock || item.stock == 0).collect()
    } else {
        inventory_items
    };

    Ok(Json(filtered_items))
}

pub async fn get_inventory_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<InventoryStats>, AppError> {
    let all_items = items::Entity::find().all(&state.db).await?;

    let total_items = all_items.len() as u64;
    let low_stock_items = all_items.iter().filter(|item| {
        let min_stock = item.min_stock.unwrap_or(0);
        item.stock <= min_stock && item.stock > 0
    }).count() as u64;

    let out_of_stock_items = all_items.iter().filter(|item| item.stock == 0).count() as u64;
    let total_stock_value = all_items.iter().map(|item| item.stock).sum::<i32>() as u64;

    Ok(Json(InventoryStats {
        total_items,
        low_stock_items,
        out_of_stock_items,
        total_stock_value,
    }))
}

pub async fn get_item_inventory(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<InventoryItem>, AppError> {
    let item = items::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("物料 {} 不存在", id)))?;

    Ok(Json(build_inventory_item(&item)))
}

pub async fn adjust_stock(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<UserClaims>,
    Path(id): Path<i32>,
    Json(payload): Json<StockAdjustmentRequest>,
) -> Result<Json<InventoryItem>, AppError> {
    let item = items::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("物料 {} 不存在", id)))?;

    if payload.quantity <= 0 {
        return Err(AppError::BadRequest("调整数量必须大于0".to_string()));
    }

    let valid_types = vec![
        ADJUSTMENT_TYPE_INBOUND,
        ADJUSTMENT_TYPE_OUTBOUND,
        ADJUSTMENT_TYPE_COUNTING,
        ADJUSTMENT_TYPE_LOSS,
        ADJUSTMENT_TYPE_RETURN,
    ];
    if !valid_types.contains(&payload.adjustment_type.as_str()) {
        return Err(AppError::BadRequest(format!("无效的调整类型: {}", payload.adjustment_type)));
    }

    let before_stock = item.stock;
    let new_stock = match payload.adjustment_type.as_str() {
        ADJUSTMENT_TYPE_INBOUND | ADJUSTMENT_TYPE_COUNTING | ADJUSTMENT_TYPE_RETURN => before_stock + payload.quantity,
        ADJUSTMENT_TYPE_OUTBOUND | ADJUSTMENT_TYPE_LOSS => {
            if before_stock < payload.quantity {
                return Err(AppError::BadRequest(format!(
                    "库存不足，当前库存: {}, 请求出库: {}",
                    before_stock, payload.quantity
                )));
            }
            before_stock - payload.quantity
        },
        _ => return Err(AppError::BadRequest("无效的调整类型".to_string())),
    };

    let mut item_model: items::ActiveModel = item.clone().into();
    item_model.stock = Set(new_stock);

    let updated_item = item_model.update(&state.db).await?;

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let adjustment_type = payload.adjustment_type.clone();
    let quantity = payload.quantity;

    let adjustment_record = inventory_adjustment::ActiveModel {
        item_id: Set(id),
        adjustment_type: Set(payload.adjustment_type),
        quantity: Set(payload.quantity),
        before_stock: Set(before_stock),
        after_stock: Set(new_stock),
        reason: Set(payload.reason),
        created_by: Set(claims.user_id),
        created_at: Set(now),
        ..Default::default()
    };

    adjustment_record.insert(&state.db).await?;

    tracing::info!(
        "库存调整 - 物料ID: {}, 类型: {}, 数量: {}, 调整前: {}, 调整后: {}",
        id, adjustment_type, quantity, before_stock, new_stock
    );

    Ok(Json(build_inventory_item(&updated_item)))
}

pub async fn get_adjustment_history(
    State(state): State<Arc<AppState>>,
    Path(item_id): Path<i32>,
) -> Result<Json<Vec<inventory_adjustment::Model>>, AppError> {
    let adjustments = inventory_adjustment::Entity::find()
        .filter(inventory_adjustment::Column::ItemId.eq(item_id))
        .order_by_desc(inventory_adjustment::Column::CreatedAt)
        .all(&state.db)
        .await?;

    Ok(Json(adjustments))
}

pub async fn batch_adjust_stock(
    State(_state): State<Arc<AppState>>,
    Extension(_claims): Extension<UserClaims>,
    Json(_payload): Json<Vec<StockAdjustmentRequest>>,
) -> Result<StatusCode, AppError> {
    Err(AppError::BadRequest("批量调整功能暂未实现".to_string()))
}
