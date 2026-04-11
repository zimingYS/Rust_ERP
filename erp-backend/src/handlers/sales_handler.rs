/*
    销售管理接口
*/
use std::sync::Arc;
use axum::extract::{Path, State, Extension};
use axum::http::StatusCode;
use axum::Json;
use chrono::Local;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set};
use serde::Deserialize;
use crate::entities::{sales_order, sales_order_item};
use crate::handlers::error::AppError;
use crate::state::AppState;
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use crate::auth::UserClaims;

#[derive(Deserialize)]
pub struct CreateSalesOrderRequest{
    pub customer: String,
    pub delivery_date: String,
    pub items: Vec<SalesOrderItemRequest>
}

#[derive(Deserialize)]
pub struct SalesOrderItemRequest{
    pub item_name: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub remark: Option<String>
}

#[derive(Deserialize)]
pub struct UpdateSalesOrderRequest {
    pub customer: String,
    pub delivery_date: String,
    pub status: String,
    pub items: Vec<SalesOrderItemRequest>,
}

pub async fn get_sales_orders(
    State(state): State<Arc<AppState>>
) -> Result<Json<Vec<sales_order::Model>>, AppError> {
    let orders = sales_order::Entity::find()
        .order_by_desc(sales_order::Column::CreatedAt)
        .all(&state.db)
        .await?;

    Ok(Json(orders))
}

pub async fn get_sales_order_with_items(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<(sales_order::Model, Vec<sales_order_item::Model>)>, AppError> {
    let order = sales_order::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("销售订单 {} 不存在", id)))?;

    let items = sales_order_item::Entity::find()
        .filter(sales_order_item::Column::SalesOrderId.eq(id))
        .all(&state.db)
        .await?;

    Ok(Json((order, items)))
}

pub async fn create_sales_order(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<UserClaims>,
    Json(payload): Json<CreateSalesOrderRequest>,
) -> Result<(StatusCode, Json<sales_order::Model>), AppError> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let order_number = format!("SO{}", Local::now().format("%Y%m%d%H%M%S"));

    let total_amount: f64 = payload.items.iter()
        .map(|item| item.quantity as f64 * item.unit_price)
        .sum();

    let new_order = sales_order::ActiveModel {
        order_number: Set(order_number),
        customer: Set(payload.customer),
        order_date: Set(now.clone()),
        delivery_date: Set(payload.delivery_date),
        status: Set("待确认".to_string()),
        total_amount: Set(total_amount),
        created_by: Set(claims.user_id),
        created_at: Set(now.clone()),
        updated_at: Set(now),
        ..Default::default()
    };

    let inserted_order = new_order.insert(&state.db).await?;

    for item in &payload.items {
        let order_item = sales_order_item::ActiveModel {
            sales_order_id: Set(inserted_order.id),
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

pub async fn update_sales_order(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateSalesOrderRequest>,
) -> Result<StatusCode, AppError> {
    let order = sales_order::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("销售订单 {} 不存在", id)))?;

    let mut order: sales_order::ActiveModel = order.into();

    let total_amount: f64 = payload.items.iter()
        .map(|item| item.quantity as f64 * item.unit_price)
        .sum();

    order.customer = Set(payload.customer);
    order.delivery_date = Set(payload.delivery_date);
    order.status = Set(payload.status);
    order.total_amount = Set(total_amount);
    order.updated_at = Set(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

    order.update(&state.db).await?;

    sales_order_item::Entity::delete_many()
        .filter(sales_order_item::Column::SalesOrderId.eq(id))
        .exec(&state.db)
        .await?;

    for item in &payload.items {
        let order_item = sales_order_item::ActiveModel {
            sales_order_id: Set(id),
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

pub async fn delete_sales_order(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    sales_order_item::Entity::delete_many()
        .filter(sales_order_item::Column::SalesOrderId.eq(id))
        .exec(&state.db)
        .await?;

    let res = sales_order::Entity::delete_by_id(id)
        .exec(&state.db)
        .await?;

    if res.rows_affected > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound(format!("销售订单 {} 不存在", id)))
    }
}

pub async fn confirm_sales_order(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let order = sales_order::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("销售订单 {} 不存在", id)))?;

    let mut order: sales_order::ActiveModel = order.into();
    order.status = Set("已确认".to_string());
    order.updated_at = Set(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

    order.update(&state.db).await?;
    Ok(StatusCode::OK)
}