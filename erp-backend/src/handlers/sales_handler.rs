/*
    销售管理接口
*/
use std::sync::Arc;
use axum::extract::{Path, Query, State, Extension};
use axum::http::StatusCode;
use axum::Json;
use chrono::Local;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set, QueryFilter, ColumnTrait};
use serde::{Deserialize, Serialize};
use crate::entities::{sales_order, sales_order_item, financial_transaction, items, inventory_adjustment};
use crate::handlers::error::AppError;
use crate::state::AppState;
use crate::auth::UserClaims;
use crate::constants::{
    SALES_ORDER_STATUS_PENDING_CONFIRM,
    SALES_ORDER_STATUS_CONFIRMED,
    SALES_ORDER_STATUS_COMPLETED,
    ORDER_STATUS_CANCELLED,
    SALES_ORDER_PREFIX,
};
use crate::permissions::can_approve;

#[derive(Deserialize)]
pub struct CreateSalesOrderRequest{
    pub customer: String,
    pub delivery_date: String,
    pub items: Vec<SalesOrderItemRequest>
}

#[derive(Deserialize)]
pub struct SalesOrderItemRequest{
    pub item_id: Option<i32>,
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

#[derive(Deserialize)]
pub struct SalesOrderQueryParams {
    pub status: Option<String>,
    pub customer: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Serialize)]
pub struct SalesOrderStats {
    pub total_orders: u64,
    pub pending_confirm: u64,
    pub confirmed: u64,
    pub total_amount: f64,
    pub this_month_amount: f64,
}

pub async fn get_sales_orders(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SalesOrderQueryParams>,
) -> Result<Json<Vec<sales_order::Model>>, AppError> {
    let mut query = sales_order::Entity::find();

    if let Some(status) = &params.status {
        query = query.filter(sales_order::Column::Status.eq(status));
    }

    if let Some(customer) = &params.customer {
        query = query.filter(sales_order::Column::Customer.contains(customer));
    }

    if let Some(start_date) = &params.start_date {
        query = query.filter(sales_order::Column::OrderDate.gte(start_date));
    }

    if let Some(end_date) = &params.end_date {
        query = query.filter(sales_order::Column::OrderDate.lte(end_date));
    }

    let orders = query
        .order_by_desc(sales_order::Column::CreatedAt)
        .all(&state.db)
        .await?;

    Ok(Json(orders))
}

pub async fn get_sales_order_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<SalesOrderStats>, AppError> {
    let all_orders = sales_order::Entity::find().all(&state.db).await?;

    let total_orders = all_orders.len() as u64;
    let pending_confirm = all_orders.iter().filter(|o| o.status == SALES_ORDER_STATUS_PENDING_CONFIRM).count() as u64;
    let confirmed = all_orders.iter().filter(|o| o.status == SALES_ORDER_STATUS_CONFIRMED).count() as u64;
    let total_amount: f64 = all_orders.iter().map(|o| o.total_amount).sum();

    let now = Local::now();
    let current_month = now.format("%Y-%m").to_string();
    let this_month_amount: f64 = all_orders.iter()
        .filter(|o| o.order_date.starts_with(&current_month))
        .map(|o| o.total_amount)
        .sum();

    Ok(Json(SalesOrderStats {
        total_orders,
        pending_confirm,
        confirmed,
        total_amount,
        this_month_amount,
    }))
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
    if payload.items.is_empty() {
        return Err(AppError::BadRequest("订单项不能为空".to_string()));
    }

    for item in &payload.items {
        if item.quantity <= 0 {
            return Err(AppError::BadRequest(format!("物品 {} 的数量必须大于0", item.item_name)));
        }
        if item.unit_price < 0.0 {
            return Err(AppError::BadRequest(format!("物品 {} 的单价不能为负数", item.item_name)));
        }
    }

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let order_number = format!("{}{}", SALES_ORDER_PREFIX, Local::now().format("%Y%m%d%H%M%S"));

    let total_amount: f64 = payload.items.iter()
        .map(|item| item.quantity as f64 * item.unit_price)
        .sum();

    let new_order = sales_order::ActiveModel {
        order_number: Set(order_number),
        customer: Set(payload.customer),
        order_date: Set(now.clone()),
        delivery_date: Set(payload.delivery_date),
        status: Set(SALES_ORDER_STATUS_PENDING_CONFIRM.to_string()),
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
            item_id: Set(item.item_id),
            item_name: Set(item.item_name.clone()),
            quantity: Set(item.quantity),
            unit_price: Set(item.unit_price),
            total_price: Set(item.quantity as f64 * item.unit_price),
            remark: Set(item.remark.clone()),
            ..Default::default()
        };
        order_item.insert(&state.db).await?;
    }

    tracing::info!("创建销售订单成功: {}", inserted_order.order_number);
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

    if order.status == SALES_ORDER_STATUS_CONFIRMED || order.status == SALES_ORDER_STATUS_COMPLETED {
        return Err(AppError::BadRequest("已确认或已完成的订单不能修改".to_string()));
    }

    if payload.items.is_empty() {
        return Err(AppError::BadRequest("订单项不能为空".to_string()));
    }

    for item in &payload.items {
        if item.quantity <= 0 {
            return Err(AppError::BadRequest(format!("物品 {} 的数量必须大于0", item.item_name)));
        }
        if item.unit_price < 0.0 {
            return Err(AppError::BadRequest(format!("物品 {} 的单价不能为负数", item.item_name)));
        }
    }

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
            item_id: Set(item.item_id),
            item_name: Set(item.item_name.clone()),
            quantity: Set(item.quantity),
            unit_price: Set(item.unit_price),
            total_price: Set(item.quantity as f64 * item.unit_price),
            remark: Set(item.remark.clone()),
            ..Default::default()
        };
        order_item.insert(&state.db).await?;
    }

    tracing::info!("更新销售订单成功: ID={}", id);
    Ok(StatusCode::OK)
}

pub async fn delete_sales_order(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let order = sales_order::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("销售订单 {} 不存在", id)))?;

    if order.status == SALES_ORDER_STATUS_CONFIRMED || order.status == SALES_ORDER_STATUS_COMPLETED {
        return Err(AppError::BadRequest("已确认或已完成的订单不能删除".to_string()));
    }

    sales_order_item::Entity::delete_many()
        .filter(sales_order_item::Column::SalesOrderId.eq(id))
        .exec(&state.db)
        .await?;

    let res = sales_order::Entity::delete_by_id(id)
        .exec(&state.db)
        .await?;

    if res.rows_affected > 0 {
        tracing::info!("删除销售订单成功: ID={}", id);
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound(format!("销售订单 {} 不存在", id)))
    }
}

pub async fn confirm_sales_order(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<UserClaims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    if !can_approve(claims.role_id) {
        return Err(AppError::Forbidden("您没有确认订单权限".to_string()));
    }

    let order = sales_order::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("销售订单 {} 不存在", id)))?;

    if order.status != SALES_ORDER_STATUS_PENDING_CONFIRM {
        return Err(AppError::BadRequest(format!("订单当前状态为 '{}', 无法确认", order.status)));
    }

    let mut order: sales_order::ActiveModel = order.into();
    order.status = Set(SALES_ORDER_STATUS_CONFIRMED.to_string());
    order.updated_at = Set(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

    order.update(&state.db).await?;
    tracing::info!("确认销售订单成功: ID={}, 确认人: {}", id, claims.username);
    Ok(StatusCode::OK)
}

pub async fn complete_sales_order(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<UserClaims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    if !can_approve(claims.role_id) {
        return Err(AppError::Forbidden("您没有完成订单权限".to_string()));
    }

    let order = sales_order::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("销售订单 {} 不存在", id)))?;

    if order.status != SALES_ORDER_STATUS_CONFIRMED {
        return Err(AppError::BadRequest(format!("订单当前状态为 '{}', 无法完成", order.status)));
    }

    let order_items = sales_order_item::Entity::find()
        .filter(sales_order_item::Column::SalesOrderId.eq(id))
        .all(&state.db)
        .await?;

    for item in &order_items {
        if let Some(item_id) = item.item_id {
            let inventory_item = items::Entity::find_by_id(item_id)
                .one(&state.db)
                .await?
                .ok_or_else(|| AppError::NotFound(format!("物料 {} 不存在", item_id)))?;

            let before_stock = inventory_item.stock;

            if before_stock < item.quantity {
                return Err(AppError::BadRequest(format!(
                    "物料 '{}' 库存不足,当前库存: {}, 需要出库: {}",
                    inventory_item.name, before_stock, item.quantity
                )));
            }

            let new_stock = before_stock - item.quantity;

            let mut item_model: items::ActiveModel = inventory_item.into();
            item_model.stock = Set(new_stock);
            item_model.update(&state.db).await?;

            let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let adjustment_record = inventory_adjustment::ActiveModel {
                item_id: Set(item_id),
                adjustment_type: Set("销售出库".to_string()),
                quantity: Set(item.quantity),
                before_stock: Set(before_stock),
                after_stock: Set(new_stock),
                reason: Set(Some(format!("销售订单: {}", order.order_number))),
                created_by: Set(claims.user_id),
                created_at: Set(now),
                ..Default::default()
            };
            adjustment_record.insert(&state.db).await?;

            tracing::info!(
                "销售出库 - 物料ID: {}, 数量: {}, 调整前: {}, 调整后: {}",
                item_id, item.quantity, before_stock, new_stock
            );
        }
    }

    let mut order: sales_order::ActiveModel = order.into();
    order.status = Set(SALES_ORDER_STATUS_COMPLETED.to_string());
    order.updated_at = Set(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

    let updated_order = order.update(&state.db).await?;

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let transaction = financial_transaction::ActiveModel {
        transaction_type: Set("收入".to_string()),
        category: Set("销售收入".to_string()),
        amount: Set(updated_order.total_amount),
        description: Set(Some(format!("销售订单: {} - 客户: {}", updated_order.order_number, updated_order.customer))),
        transaction_date: Set(updated_order.order_date.clone()),
        related_order_id: Set(Some(id)),
        related_order_type: Set(Some("销售订单".to_string())),
        created_by: Set(claims.user_id),
        created_at: Set(now),
        ..Default::default()
    };

    transaction.insert(&state.db).await?;

    tracing::info!("完成销售订单并生成财务记录: ID={}, 金额: {}, 操作人: {}", id, updated_order.total_amount, claims.username);
    Ok(StatusCode::OK)
}

pub async fn cancel_sales_order(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<UserClaims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    if !can_approve(claims.role_id) {
        return Err(AppError::Forbidden("您没有取消订单权限".to_string()));
    }

    let order = sales_order::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("销售订单 {} 不存在", id)))?;

    if order.status == SALES_ORDER_STATUS_CONFIRMED || order.status == SALES_ORDER_STATUS_COMPLETED {
        return Err(AppError::BadRequest("已确认或已完成的订单不能取消".to_string()));
    }

    let mut order: sales_order::ActiveModel = order.into();
    order.status = Set(ORDER_STATUS_CANCELLED.to_string());
    order.updated_at = Set(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

    order.update(&state.db).await?;
    tracing::info!("取消销售订单成功: ID={}, 操作人: {}", id, claims.username);
    Ok(StatusCode::OK)
}