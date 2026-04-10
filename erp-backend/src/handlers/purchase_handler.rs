/*
    采购管理接口
*/
use std::sync::Arc;
use axum::extract::{Path, State, Extension};
use axum::http::StatusCode;
use axum::Json;
use chrono::Local;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set};
use serde::Deserialize;
use crate::entities::{purchase_order, purchase_order_item};
use crate::handlers::error::AppError;
use crate::state::AppState;
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use crate::auth::UserClaims;

#[derive(Deserialize)]
pub struct CreatePurchaseOrderRequest{
    pub supplier: String,
    pub expected_delivery_date: String,
    pub items: Vec<PurchaseOrderItemRequest>
}

#[derive(Deserialize)]
pub struct PurchaseOrderItemRequest{
    pub item_name: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub remark: Option<String>
}

#[derive(Deserialize)]
pub struct UpdatePurchaseOrderRequest {
    pub supplier: String,
    pub expected_delivery_date: String,
    pub status: String,
    pub items: Vec<PurchaseOrderItemRequest>,
}

pub async fn get_purchase_orders(
    State(state): State<Arc<AppState>>
) -> Result<Json<Vec<purchase_order::Model>>, AppError> {
    let orders = purchase_order::Entity::find()
        .order_by_desc(purchase_order::Column::CreatedAt)
        .all(&state.db)
        .await?;

    Ok(Json(orders))
}

pub async fn get_purchase_order_with_items(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<(purchase_order::Model, Vec<purchase_order_item::Model>)>, AppError> {
    let order = purchase_order::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("采购订单 {} 不存在", id)))?;

    let items = purchase_order_item::Entity::find()
        .filter(purchase_order_item::Column::PurchaseOrderId.eq(id))
        .all(&state.db)
        .await?;

    Ok(Json((order, items)))
}

pub async fn create_purchase_order(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<UserClaims>,
    Json(payload): Json<CreatePurchaseOrderRequest>,
) -> Result<(StatusCode, Json<purchase_order::Model>), AppError> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let order_number = format!("PO{}", Local::now().format("%Y%m%d%H%M%S"));

    let total_amount: f64 = payload.items.iter()
        .map(|item| item.quantity as f64 * item.unit_price)
        .sum();

    let new_order = purchase_order::ActiveModel {
        order_number: Set(order_number),
        supplier: Set(payload.supplier),
        order_date: Set(now.clone()),
        expected_delivery_date: Set(payload.expected_delivery_date),
        status: Set("待审批".to_string()),
        total_amount: Set(total_amount),
        created_by: Set(claims.user_id),
        created_at: Set(now.clone()),
        updated_at: Set(now),
        ..Default::default()
    };

    let inserted_order = new_order.insert(&state.db).await?;

    for item in &payload.items {
        let order_item = purchase_order_item::ActiveModel {
            purchase_order_id: Set(inserted_order.id),
            item_name: Set(item.item_name.clone()),
            quantity: Set(item.quantity),
            unit_price: Set(item.unit_price),
            total_price: Set(item.quantity as f64 * item.unit_price),
            remark: Set(item.remark.clone()),
            ..Default::default()
        };
        order_item.insert(&state.db).await?;
    }

    Ok((StatusCode::CREATED, Json(inserted_order)))
}

pub async fn update_purchase_order(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePurchaseOrderRequest>,
) -> Result<StatusCode, AppError> {
    let order = purchase_order::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("采购订单 {} 不存在", id)))?;

    let mut order: purchase_order::ActiveModel = order.into();

    let total_amount: f64 = payload.items.iter()
        .map(|item| item.quantity as f64 * item.unit_price)
        .sum();

    order.supplier = Set(payload.supplier);
    order.expected_delivery_date = Set(payload.expected_delivery_date);
    order.status = Set(payload.status);
    order.total_amount = Set(total_amount);
    order.updated_at = Set(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

    order.update(&state.db).await?;

    purchase_order_item::Entity::delete_many()
        .filter(purchase_order_item::Column::PurchaseOrderId.eq(id))
        .exec(&state.db)
        .await?;

    for item in &payload.items {
        let order_item = purchase_order_item::ActiveModel {
            purchase_order_id: Set(id),
            item_name: Set(item.item_name.clone()),
            quantity: Set(item.quantity),
            unit_price: Set(item.unit_price),
            total_price: Set(item.quantity as f64 * item.unit_price),
            remark: Set(item.remark.clone()),
            ..Default::default()
        };
        order_item.insert(&state.db).await?;
    }

    Ok(StatusCode::OK)
}

pub async fn delete_purchase_order(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    purchase_order_item::Entity::delete_many()
        .filter(purchase_order_item::Column::PurchaseOrderId.eq(id))
        .exec(&state.db)
        .await?;

    let res = purchase_order::Entity::delete_by_id(id)
        .exec(&state.db)
        .await?;

    if res.rows_affected > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound(format!("采购订单 {} 不存在", id)))
    }
}

pub async fn approve_purchase_order(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let order = purchase_order::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("采购订单 {} 不存在", id)))?;

    let mut order: purchase_order::ActiveModel = order.into();
    order.status = Set("已审批".to_string());
    order.updated_at = Set(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

    order.update(&state.db).await?;
    Ok(StatusCode::OK)
}
