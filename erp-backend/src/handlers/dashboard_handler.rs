/*
    仪表盘数据接口
*/
use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use chrono::{Local, Datelike};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, QueryOrder, PaginatorTrait, FromQueryResult, QuerySelect};
use serde::Serialize;
use crate::entities::{purchase_order, sales_order, items, financial_transaction, sales_order_item};
use crate::handlers::error::AppError;
use crate::state::AppState;

#[derive(Serialize)]
pub struct DashboardStats {
    pub today_sales: f64,
    pub today_purchases: f64,
    pub month_sales: f64,
    pub month_purchases: f64,
    pub total_orders: u64,
    pub pending_orders: u64,
    pub low_stock_count: u64,
    pub total_revenue: f64,
    pub total_expenses: f64,
}

#[derive(Serialize)]
pub struct SalesTrendData {
    pub date: String,
    pub amount: f64,
    pub order_count: u64,
}

#[derive(Serialize)]
pub struct CategoryDistribution {
    pub category: String,
    pub amount: f64,
    pub percentage: f64,
}

#[derive(Serialize, FromQueryResult)]
pub struct TopProduct {
    pub item_name: String,
    pub total_quantity: i64,
    pub total_amount: f64,
}

pub async fn get_dashboard_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<DashboardStats>, AppError> {
    let now = Local::now();
    let today = now.format("%Y-%m-%d").to_string();
    let month_start = now.with_day(1).unwrap().format("%Y-%m-%d").to_string();

    // 今日销售
    let today_sales_orders = sales_order::Entity::find()
        .filter(sales_order::Column::OrderDate.gte(today.clone()))
        .filter(sales_order::Column::OrderDate.lt(now.format("%Y-%m-%d %H:%M:%S").to_string()))
        .all(&state.db)
        .await?;
    let today_sales = today_sales_orders.iter().map(|o| o.total_amount).sum();

    // 今日采购
    let today_purchase_orders = purchase_order::Entity::find()
        .filter(purchase_order::Column::OrderDate.gte(today.clone()))
        .filter(purchase_order::Column::OrderDate.lt(now.format("%Y-%m-%d %H:%M:%S").to_string()))
        .all(&state.db)
        .await?;
    let today_purchases = today_purchase_orders.iter().map(|o| o.total_amount).sum();

    // 本月销售
    let month_sales_orders = sales_order::Entity::find()
        .filter(sales_order::Column::OrderDate.gte(month_start.clone()))
        .all(&state.db)
        .await?;
    let month_sales = month_sales_orders.iter().map(|o| o.total_amount).sum();

    // 本月采购
    let month_purchase_orders = purchase_order::Entity::find()
        .filter(purchase_order::Column::OrderDate.gte(month_start.clone()))
        .all(&state.db)
        .await?;
    let month_purchases = month_purchase_orders.iter().map(|o| o.total_amount).sum();

    // 订单统计
    let total_orders = sales_order::Entity::find().count(&state.db).await?
        + purchase_order::Entity::find().count(&state.db).await?;

    let pending_orders = sales_order::Entity::find()
        .filter(sales_order::Column::Status.eq("待确认"))
        .count(&state.db)
        .await?
        + purchase_order::Entity::find()
        .filter(purchase_order::Column::Status.eq("待审批"))
        .count(&state.db)
        .await?;

    // 低库存统计
    let all_items = items::Entity::find().all(&state.db).await?;
    let low_stock_count = all_items.iter()
        .filter(|item| {
            let min_stock = item.min_stock.unwrap_or(0);
            item.stock <= min_stock
        })
        .count() as u64;

    // 财务统计
    let transactions = financial_transaction::Entity::find().all(&state.db).await?;
    let total_revenue = transactions.iter()
        .filter(|t| t.transaction_type == "收入")
        .map(|t| t.amount)
        .sum();
    let total_expenses = transactions.iter()
        .filter(|t| t.transaction_type == "支出")
        .map(|t| t.amount)
        .sum();

    Ok(Json(DashboardStats {
        today_sales,
        today_purchases,
        month_sales,
        month_purchases,
        total_orders,
        pending_orders,
        low_stock_count,
        total_revenue,
        total_expenses,
    }))
}

pub async fn get_sales_trend(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SalesTrendData>>, AppError> {
    let now = Local::now();
    let mut trend_data = Vec::new();

    // 获取最近7天的销售数据
    for i in (0..7).rev() {
        let date = now.clone() - chrono::Duration::days(i);
        let date_str = date.format("%Y-%m-%d").to_string();
        let next_date = (date.clone() + chrono::Duration::days(1)).format("%Y-%m-%d").to_string();

        let orders = sales_order::Entity::find()
            .filter(sales_order::Column::OrderDate.gte(date_str.clone()))
            .filter(sales_order::Column::OrderDate.lt(next_date))
            .all(&state.db)
            .await?;

        let amount = orders.iter().map(|o| o.total_amount).sum();
        let order_count = orders.len() as u64;

        trend_data.push(SalesTrendData {
            date: date.format("%m-%d").to_string(),
            amount,
            order_count,
        });
    }

    Ok(Json(trend_data))
}

pub async fn get_purchase_trend(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SalesTrendData>>, AppError> {
    let now = Local::now();
    let mut trend_data = Vec::new();

    // 获取最近7天的采购数据
    for i in (0..7).rev() {
        let date = now.clone() - chrono::Duration::days(i);
        let date_str = date.format("%Y-%m-%d").to_string();
        let next_date = (date.clone() + chrono::Duration::days(1)).format("%Y-%m-%d").to_string();

        let orders = purchase_order::Entity::find()
            .filter(purchase_order::Column::OrderDate.gte(date_str.clone()))
            .filter(purchase_order::Column::OrderDate.lt(next_date))
            .all(&state.db)
            .await?;

        let amount = orders.iter().map(|o| o.total_amount).sum();
        let order_count = orders.len() as u64;

        trend_data.push(SalesTrendData {
            date: date.format("%m-%d").to_string(),
            amount,
            order_count,
        });
    }

    Ok(Json(trend_data))
}

pub async fn get_inventory_distribution(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<CategoryDistribution>>, AppError> {
    let all_items = items::Entity::find().all(&state.db).await?;

    let normal_count = all_items.iter()
        .filter(|item| {
            let min_stock = item.min_stock.unwrap_or(0);
            item.stock > min_stock
        })
        .count() as f64;

    let low_stock_count = all_items.iter()
        .filter(|item| {
            let min_stock = item.min_stock.unwrap_or(0);
            item.stock > 0 && item.stock <= min_stock
        })
        .count() as f64;

    let out_of_stock_count = all_items.iter()
        .filter(|item| item.stock == 0)
        .count() as f64;

    let total = all_items.len() as f64;

    Ok(Json(vec![
        CategoryDistribution {
            category: "正常".to_string(),
            amount: normal_count,
            percentage: if total > 0.0 { (normal_count / total) * 100.0 } else { 0.0 },
        },
        CategoryDistribution {
            category: "低库存".to_string(),
            amount: low_stock_count,
            percentage: if total > 0.0 { (low_stock_count / total) * 100.0 } else { 0.0 },
        },
        CategoryDistribution {
            category: "缺货".to_string(),
            amount: out_of_stock_count,
            percentage: if total > 0.0 { (out_of_stock_count / total) * 100.0 } else { 0.0 },
        },
    ]))
}

pub async fn get_top_products(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TopProduct>>, AppError> {
    // 获取热销产品（基于销售订单项）
    let top_items = sales_order_item::Entity::find()
        .select_only()
        .column(sales_order_item::Column::ItemName)
        .column_as(sales_order_item::Column::Quantity.sum(), "total_quantity")
        .column_as(sales_order_item::Column::TotalPrice.sum(), "total_amount")
        .group_by(sales_order_item::Column::ItemName)
        .order_by_desc(sales_order_item::Column::Quantity.sum())
        .limit(10)
        .into_model::<TopProduct>()
        .all(&state.db)
        .await?;

    Ok(Json(top_items))
}

pub async fn get_financial_summary(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<CategoryDistribution>>, AppError> {
    let transactions = financial_transaction::Entity::find().all(&state.db).await?;

    let income = transactions.iter()
        .filter(|t| t.transaction_type == "收入")
        .map(|t| t.amount)
        .sum();

    let expense = transactions.iter()
        .filter(|t| t.transaction_type == "支出")
        .map(|t| t.amount)
        .sum();

    let total = income + expense;

    Ok(Json(vec![
        CategoryDistribution {
            category: "收入".to_string(),
            amount: income,
            percentage: if total > 0.0 { (income / total) * 100.0 } else { 0.0 },
        },
        CategoryDistribution {
            category: "支出".to_string(),
            amount: expense,
            percentage: if total > 0.0 { (expense / total) * 100.0 } else { 0.0 },
        },
    ]))
}