/*
    采购管理接口
*/
use std::sync::Arc;
use axum::extract::{Path, Query, State, Extension};
use axum::http::StatusCode;
use axum::Json;
use chrono::Local;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set, QueryFilter, ColumnTrait};
use serde::{Deserialize, Serialize};
use crate::entities::{purchase_order, purchase_order_item, financial_transaction};
use crate::handlers::error::AppError;
use crate::state::AppState;
use crate::auth::UserClaims;
use crate::constants::{
    ORDER_STATUS_PENDING_APPROVAL,
    ORDER_STATUS_APPROVED,
    ORDER_STATUS_CANCELLED,
    PURCHASE_ORDER_PREFIX,
};
use crate::permissions::can_approve;

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

#[derive(Deserialize)]
pub struct PurchaseOrderQueryParams {
    pub status: Option<String>,
    pub supplier: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Serialize)]
pub struct PurchaseOrderStats {
    pub total_orders: u64,
    pub pending_approval: u64,
    pub approved: u64,
    pub total_amount: f64,
    pub this_month_amount: f64,
}

fn validate_purchase_order_items(items: &[PurchaseOrderItemRequest]) -> Result<(), AppError> {
    if items.is_empty() {
        return Err(AppError::BadRequest("订单项不能为空".to_string()));
    }

    for item in items {
        if item.quantity <= 0 {
            return Err(AppError::BadRequest(format!("物品 {} 的数量必须大于0", item.item_name)));
        }
        if item.unit_price < 0.0 {
            return Err(AppError::BadRequest(format!("物品 {} 的单价不能为负数", item.item_name)));
        }
    }

    Ok(())
}

pub async fn get_purchase_orders(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PurchaseOrderQueryParams>,
) -> Result<Json<Vec<purchase_order::Model>>, AppError> {
    let mut query = purchase_order::Entity::find();

    if let Some(status) = &params.status {
        query = query.filter(purchase_order::Column::Status.eq(status));
    }

    if let Some(supplier) = &params.supplier {
        query = query.filter(purchase_order::Column::Supplier.contains(supplier));
    }

    if let Some(start_date) = &params.start_date {
        query = query.filter(purchase_order::Column::OrderDate.gte(start_date));
    }

    if let Some(end_date) = &params.end_date {
        query = query.filter(purchase_order::Column::OrderDate.lte(end_date));
    }

    let orders = query
        .order_by_desc(purchase_order::Column::CreatedAt)
        .all(&state.db)
        .await?;

    Ok(Json(orders))
}

pub async fn get_purchase_order_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<PurchaseOrderStats>, AppError> {
    let all_orders = purchase_order::Entity::find().all(&state.db).await?;

    let total_orders = all_orders.len() as u64;
    let pending_approval = all_orders.iter().filter(|o| o.status == ORDER_STATUS_PENDING_APPROVAL).count() as u64;
    let approved = all_orders.iter().filter(|o| o.status == ORDER_STATUS_APPROVED).count() as u64;
    let total_amount: f64 = all_orders.iter().map(|o| o.total_amount).sum();

    let now = Local::now();
    let current_month = now.format("%Y-%m").to_string();
    let this_month_amount: f64 = all_orders.iter()
        .filter(|o| o.order_date.starts_with(&current_month))
        .map(|o| o.total_amount)
        .sum();

    Ok(Json(PurchaseOrderStats {
        total_orders,
        pending_approval,
        approved,
        total_amount,
        this_month_amount,
    }))
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
    validate_purchase_order_items(&payload.items)?;

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let order_number = format!("{}{}", PURCHASE_ORDER_PREFIX, Local::now().format("%Y%m%d%H%M%S"));

    let total_amount: f64 = payload.items.iter()
        .map(|item| item.quantity as f64 * item.unit_price)
        .sum();

    let new_order = purchase_order::ActiveModel {
        order_number: Set(order_number),
        supplier: Set(payload.supplier),
        order_date: Set(now.clone()),
        expected_delivery_date: Set(payload.expected_delivery_date),
        status: Set(ORDER_STATUS_PENDING_APPROVAL.to_string()),
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

    tracing::info!("创建采购订单成功: {}", inserted_order.order_number);
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

    if order.status == ORDER_STATUS_APPROVED {
        return Err(AppError::BadRequest("已审批的订单不能修改".to_string()));
    }

    validate_purchase_order_items(&payload.items)?;

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

    tracing::info!("更新采购订单成功: ID={}", id);
    Ok(StatusCode::OK)
}

pub async fn delete_purchase_order(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let order = purchase_order::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("采购订单 {} 不存在", id)))?;

    if order.status == ORDER_STATUS_APPROVED {
        return Err(AppError::BadRequest("已审批的订单不能删除".to_string()));
    }

    purchase_order_item::Entity::delete_many()
        .filter(purchase_order_item::Column::PurchaseOrderId.eq(id))
        .exec(&state.db)
        .await?;

    let res = purchase_order::Entity::delete_by_id(id)
        .exec(&state.db)
        .await?;

    if res.rows_affected > 0 {
        tracing::info!("删除采购订单成功: ID={}", id);
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound(format!("采购订单 {} 不存在", id)))
    }
}

pub async fn approve_purchase_order(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<UserClaims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    if !can_approve(claims.role_id) {
        return Err(AppError::Forbidden("您没有审批权限".to_string()));
    }

    let order = purchase_order::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("采购订单 {} 不存在", id)))?;

    if order.status != ORDER_STATUS_PENDING_APPROVAL {
        return Err(AppError::BadRequest(format!("订单当前状态为 '{}', 无法审批", order.status)));
    }

    let mut order: purchase_order::ActiveModel = order.into();
    order.status = Set(ORDER_STATUS_APPROVED.to_string());
    order.updated_at = Set(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

    let updated_order = order.update(&state.db).await?;

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let transaction = financial_transaction::ActiveModel {
        transaction_type: Set("支出".to_string()),
        category: Set("采购成本".to_string()),
        amount: Set(updated_order.total_amount),
        description: Set(Some(format!("采购订单: {} - 供应商: {}", updated_order.order_number, updated_order.supplier))),
        transaction_date: Set(updated_order.order_date.clone()),
        related_order_id: Set(Some(id)),
        related_order_type: Set(Some("采购订单".to_string())),
        created_by: Set(claims.user_id),
        created_at: Set(now),
        ..Default::default()
    };

    transaction.insert(&state.db).await?;

    tracing::info!("审批采购订单并生成财务记录: ID={}, 金额: {}, 审批人: {}", id, updated_order.total_amount, claims.username);
    Ok(StatusCode::OK)
}

pub async fn cancel_purchase_order(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<UserClaims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    if !can_approve(claims.role_id) {
        return Err(AppError::Forbidden("您没有取消订单权限".to_string()));
    }

    let order = purchase_order::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("采购订单 {} 不存在", id)))?;

    if order.status == ORDER_STATUS_APPROVED {
        return Err(AppError::BadRequest("已审批的订单不能取消".to_string()));
    }

    let mut order: purchase_order::ActiveModel = order.into();
    order.status = Set(ORDER_STATUS_CANCELLED.to_string());
    order.updated_at = Set(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

    order.update(&state.db).await?;
    tracing::info!("取消采购订单成功: ID={}, 操作人: {}", id, claims.username);
    Ok(StatusCode::OK)
}
