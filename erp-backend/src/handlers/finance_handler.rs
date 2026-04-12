/*
    财务管理接口
*/
use std::sync::Arc;
use axum::extract::{Path, Query, State, Extension};
use axum::http::StatusCode;
use axum::Json;
use chrono::Local;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set, QueryFilter, ColumnTrait};
use serde::{Deserialize, Serialize};
use crate::entities::financial_transaction;
use crate::handlers::error::AppError;
use crate::state::AppState;
use crate::auth::UserClaims;
use crate::constants::{
    ORDER_STATUS_APPROVED,
    SALES_ORDER_STATUS_COMPLETED,
};

#[derive(Deserialize)]
pub struct TransactionQueryParams {
    pub transaction_type: Option<String>,
    pub category: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateTransactionRequest {
    pub transaction_type: String,
    pub category: String,
    pub amount: f64,
    pub description: Option<String>,
    pub transaction_date: Option<String>,
    pub related_order_id: Option<i32>,
    pub related_order_type: Option<String>,
}

#[derive(Serialize)]
pub struct FinancialStats {
    pub total_income: f64,
    pub total_expense: f64,
    pub net_profit: f64,
    pub this_month_income: f64,
    pub this_month_expense: f64,
    pub this_month_profit: f64,
    pub transaction_count: u64,
}

#[derive(Serialize)]
pub struct CategorySummary {
    pub category: String,
    pub total_amount: f64,
    pub count: u64,
}

pub async fn get_transactions(
    State(state): State<Arc<AppState>>,
    Query(params): Query<TransactionQueryParams>,
) -> Result<Json<Vec<financial_transaction::Model>>, AppError> {
    let mut query = financial_transaction::Entity::find();

    if let Some(transaction_type) = &params.transaction_type {
        query = query.filter(financial_transaction::Column::TransactionType.eq(transaction_type));
    }

    if let Some(category) = &params.category {
        query = query.filter(financial_transaction::Column::Category.contains(category));
    }

    if let Some(start_date) = &params.start_date {
        query = query.filter(financial_transaction::Column::TransactionDate.gte(start_date));
    }

    if let Some(end_date) = &params.end_date {
        query = query.filter(financial_transaction::Column::TransactionDate.lte(end_date));
    }

    let transactions = query
        .order_by_desc(financial_transaction::Column::TransactionDate)
        .all(&state.db)
        .await?;

    Ok(Json(transactions))
}

pub async fn get_financial_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<FinancialStats>, AppError> {
    let all_transactions = financial_transaction::Entity::find().all(&state.db).await?;

    let total_income: f64 = all_transactions.iter()
        .filter(|t| t.transaction_type == "收入")
        .map(|t| t.amount)
        .sum();

    let total_expense: f64 = all_transactions.iter()
        .filter(|t| t.transaction_type == "支出")
        .map(|t| t.amount)
        .sum();

    let net_profit = total_income - total_expense;

    let now = Local::now();
    let current_month = now.format("%Y-%m").to_string();

    let this_month_income: f64 = all_transactions.iter()
        .filter(|t| t.transaction_type == "收入" && t.transaction_date.starts_with(&current_month))
        .map(|t| t.amount)
        .sum();

    let this_month_expense: f64 = all_transactions.iter()
        .filter(|t| t.transaction_type == "支出" && t.transaction_date.starts_with(&current_month))
        .map(|t| t.amount)
        .sum();

    let this_month_profit = this_month_income - this_month_expense;

    let transaction_count = all_transactions.len() as u64;

    Ok(Json(FinancialStats {
        total_income,
        total_expense,
        net_profit,
        this_month_income,
        this_month_expense,
        this_month_profit,
        transaction_count,
    }))
}

pub async fn get_category_summary(
    State(state): State<Arc<AppState>>,
    Path(transaction_type): Path<String>,
) -> Result<Json<Vec<CategorySummary>>, AppError> {
    let transactions = financial_transaction::Entity::find()
        .filter(financial_transaction::Column::TransactionType.eq(&transaction_type))
        .all(&state.db)
        .await?;

    let mut category_map: std::collections::HashMap<String, (f64, u64)> = std::collections::HashMap::new();

    for transaction in &transactions {
        let entry = category_map.entry(transaction.category.clone()).or_insert((0.0, 0));
        entry.0 += transaction.amount;
        entry.1 += 1;
    }

    let summary: Vec<CategorySummary> = category_map.into_iter().map(|(category, (total, count))| {
        CategorySummary {
            category,
            total_amount: total,
            count,
        }
    }).collect();

    Ok(Json(summary))
}

pub async fn create_transaction(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<UserClaims>,
    Json(payload): Json<CreateTransactionRequest>,
) -> Result<(StatusCode, Json<financial_transaction::Model>), AppError> {
    if payload.amount <= 0.0 {
        return Err(AppError::BadRequest("金额必须大于0".to_string()));
    }

    let valid_types = vec!["收入", "支出"];
    if !valid_types.contains(&payload.transaction_type.as_str()) {
        return Err(AppError::BadRequest(format!("无效的交易类型: {}", payload.transaction_type)));
    }

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let transaction_date = payload.transaction_date.unwrap_or_else(|| {
        Local::now().format("%Y-%m-%d").to_string()
    });

    let new_transaction = financial_transaction::ActiveModel {
        transaction_type: Set(payload.transaction_type),
        category: Set(payload.category),
        amount: Set(payload.amount),
        description: Set(payload.description),
        transaction_date: Set(transaction_date),
        related_order_id: Set(payload.related_order_id),
        related_order_type: Set(payload.related_order_type),
        created_by: Set(claims.user_id),
        created_at: Set(now),
        ..Default::default()
    };

    let inserted = new_transaction.insert(&state.db).await?;

    tracing::info!(
        "创建财务交易 - 类型: {}, 分类: {}, 金额: {}",
        inserted.transaction_type, inserted.category, inserted.amount
    );

    Ok((StatusCode::CREATED, Json(inserted)))
}

pub async fn delete_transaction(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let res = financial_transaction::Entity::delete_by_id(id)
        .exec(&state.db)
        .await?;

    if res.rows_affected > 0 {
        tracing::info!("删除财务交易成功: ID={}", id);
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound(format!("交易记录 {} 不存在", id)))
    }
}

#[warn(dead_code)]
pub async fn auto_create_purchase_transaction(
    State(state): State<Arc<AppState>>,
    Path(order_id): Path<i32>,
) -> Result<StatusCode, AppError> {
    use crate::entities::purchase_order;

    let order = purchase_order::Entity::find_by_id(order_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("采购订单 {} 不存在", order_id)))?;

    if order.status != ORDER_STATUS_APPROVED {
        return Err(AppError::BadRequest("只有已审批的采购订单才能生成财务记录".to_string()));
    }

    let existing = financial_transaction::Entity::find()
        .filter(financial_transaction::Column::RelatedOrderId.eq(Some(order_id)))
        .filter(financial_transaction::Column::RelatedOrderType.eq(Some("采购订单".to_string())))
        .one(&state.db)
        .await?;

    if existing.is_some() {
        return Err(AppError::BadRequest("该订单已生成财务记录".to_string()));
    }

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let transaction = financial_transaction::ActiveModel {
        transaction_type: Set("支出".to_string()),
        category: Set("采购成本".to_string()),
        amount: Set(order.total_amount),
        description: Set(Some(format!("采购订单: {} - 供应商: {}", order.order_number, order.supplier))),
        transaction_date: Set(order.order_date.clone()),
        related_order_id: Set(Some(order_id)),
        related_order_type: Set(Some("采购订单".to_string())),
        created_by: Set(order.created_by),
        created_at: Set(now),
        ..Default::default()
    };

    transaction.insert(&state.db).await?;

    tracing::info!("自动生成采购财务记录: 订单ID={}", order_id);
    Ok(StatusCode::CREATED)
}

#[warn(dead_code)]
pub async fn auto_create_sales_transaction(
    State(state): State<Arc<AppState>>,
    Path(order_id): Path<i32>,
) -> Result<StatusCode, AppError> {
    use crate::entities::sales_order;

    let order = sales_order::Entity::find_by_id(order_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("销售订单 {} 不存在", order_id)))?;

    if order.status != SALES_ORDER_STATUS_COMPLETED {
        return Err(AppError::BadRequest("只有已完成的销售订单才能生成财务记录".to_string()));
    }

    let existing = financial_transaction::Entity::find()
        .filter(financial_transaction::Column::RelatedOrderId.eq(Some(order_id)))
        .filter(financial_transaction::Column::RelatedOrderType.eq(Some("销售订单".to_string())))
        .one(&state.db)
        .await?;

    if existing.is_some() {
        return Err(AppError::BadRequest("该订单已生成财务记录".to_string()));
    }

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let transaction = financial_transaction::ActiveModel {
        transaction_type: Set("收入".to_string()),
        category: Set("销售收入".to_string()),
        amount: Set(order.total_amount),
        description: Set(Some(format!("销售订单: {} - 客户: {}", order.order_number, order.customer))),
        transaction_date: Set(order.order_date.clone()),
        related_order_id: Set(Some(order_id)),
        related_order_type: Set(Some("销售订单".to_string())),
        created_by: Set(order.created_by),
        created_at: Set(now),
        ..Default::default()
    };

    transaction.insert(&state.db).await?;

    tracing::info!("自动生成销售财务记录: 订单ID={}", order_id);
    Ok(StatusCode::CREATED)
}
