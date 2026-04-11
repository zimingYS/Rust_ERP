/*
    报表导出接口
*/
use std::sync::Arc;
use axum::extract::{Query, State};
use axum::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use axum::response::{IntoResponse, Response};
use chrono::Local;
use csv::WriterBuilder;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, QueryOrder};
use serde::Deserialize;
use crate::entities::{purchase_order, sales_order, items, financial_transaction};
use crate::handlers::error::AppError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ExportQueryParams {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub format: Option<String>,
}

pub async fn export_purchase_orders(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ExportQueryParams>,
) -> Result<Response, AppError> {
    let mut query = purchase_order::Entity::find();

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

    let format = params.format.as_deref().unwrap_or("csv");

    if format == "json" {
        let json_data = serde_json::to_string_pretty(&orders)?;
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("purchase_orders_{}.json", timestamp);

        let headers = [
            (CONTENT_TYPE, "application/json"),
            (CONTENT_DISPOSITION, &format!("attachment; filename=\"{}\"", filename)),
        ];

        Ok((headers, json_data).into_response())
    } else {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);

        wtr.write_record(&["ID", "订单编号", "供应商", "下单日期", "预计到货", "状态", "总金额", "创建人", "创建时间"])?;

        for order in &orders {
            wtr.write_record(&[
                order.id.to_string(),
                order.order_number.clone(),
                order.supplier.clone(),
                order.order_date.clone(),
                order.expected_delivery_date.clone(),
                order.status.clone(),
                format!("{:.2}", order.total_amount),
                order.created_by.to_string(),
                order.created_at.clone(),
            ])?;
        }

        let data = wtr.into_inner().map_err(|e| AppError::CsvInnerError(e.to_string()))?;
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("purchase_orders_{}.csv", timestamp);

        let headers = [
            (CONTENT_TYPE, "text/csv"),
            (CONTENT_DISPOSITION, &format!("attachment; filename=\"{}\"", filename)),
        ];

        Ok((headers, data).into_response())
    }
}

pub async fn export_sales_orders(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ExportQueryParams>,
) -> Result<Response, AppError> {
    let mut query = sales_order::Entity::find();

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

    let format = params.format.as_deref().unwrap_or("csv");

    if format == "json" {
        let json_data = serde_json::to_string_pretty(&orders)?;
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("sales_orders_{}.json", timestamp);

        let headers = [
            (CONTENT_TYPE, "application/json"),
            (CONTENT_DISPOSITION, &format!("attachment; filename=\"{}\"", filename)),
        ];

        Ok((headers, json_data).into_response())
    } else {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);

        wtr.write_record(&["ID", "订单编号", "客户", "下单日期", "交付日期", "状态", "总金额", "创建人", "创建时间"])?;

        for order in &orders {
            wtr.write_record(&[
                order.id.to_string(),
                order.order_number.clone(),
                order.customer.clone(),
                order.order_date.clone(),
                order.delivery_date.clone(),
                order.status.clone(),
                format!("{:.2}", order.total_amount),
                order.created_by.to_string(),
                order.created_at.clone(),
            ])?;
        }

        let data = wtr.into_inner().map_err(|e| AppError::CsvInnerError(e.to_string()))?;
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("sales_orders_{}.csv", timestamp);

        let headers = [
            (CONTENT_TYPE, "text/csv"),
            (CONTENT_DISPOSITION, &format!("attachment; filename=\"{}\"", filename)),
        ];

        Ok((headers, data).into_response())
    }
}

pub async fn export_inventory(
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    let items_list = items::Entity::find()
        .order_by_asc(items::Column::Id)
        .all(&state.db)
        .await?;

    let mut wtr = WriterBuilder::new().from_writer(vec![]);

    wtr.write_record(&["ID", "物料名称", "SKU", "当前库存", "单位", "最低库存", "库存状态"])?;

    for item in &items_list {
        let min_stock = item.min_stock.unwrap_or(0);
        let status = if item.stock == 0 {
            "缺货"
        } else if item.stock <= min_stock {
            "低库存"
        } else {
            "正常"
        };

        wtr.write_record(&[
            item.id.to_string(),
            item.name.clone(),
            item.sku.clone(),
            item.stock.to_string(),
            item.unit.clone().unwrap_or_default(),
            min_stock.to_string(),
            status.to_string(),
        ])?;
    }

    let data = wtr.into_inner().map_err(|e| AppError::CsvInnerError(e.to_string()))?;
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("inventory_{}.csv", timestamp);

    let headers = [
        (CONTENT_TYPE, "text/csv"),
        (CONTENT_DISPOSITION, &format!("attachment; filename=\"{}\"", filename)),
    ];

    Ok((headers, data).into_response())
}

pub async fn export_financial_transactions(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ExportQueryParams>,
) -> Result<Response, AppError> {
    let mut query = financial_transaction::Entity::find();

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

    let mut wtr = WriterBuilder::new().from_writer(vec![]);

    wtr.write_record(&["ID", "交易类型", "分类", "金额", "描述", "交易日期", "关联订单ID", "关联订单类型", "创建时间"])?;

    for transaction in &transactions {
        wtr.write_record(&[
            transaction.id.to_string(),
            transaction.transaction_type.clone(),
            transaction.category.clone(),
            format!("{:.2}", transaction.amount),
            transaction.description.clone().unwrap_or_default(),
            transaction.transaction_date.clone(),
            transaction.related_order_id.map(|id| id.to_string()).unwrap_or_default(),
            transaction.related_order_type.clone().unwrap_or_default(),
            transaction.created_at.clone(),
        ])?;
    }

    let data = wtr.into_inner().map_err(|e| AppError::CsvInnerError(e.to_string()))?;
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("financial_transactions_{}.csv", timestamp);

    let headers = [
        (CONTENT_TYPE, "text/csv"),
        (CONTENT_DISPOSITION, &format!("attachment; filename=\"{}\"", filename)),
    ];

    Ok((headers, data).into_response())
}

pub async fn export_audit_logs(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ExportQueryParams>,
) -> Result<Response, AppError> {
    use crate::entities::audit_log;

    let mut query = audit_log::Entity::find();

    if let Some(start_date) = &params.start_date {
        query = query.filter(audit_log::Column::CreatedAt.gte(start_date));
    }

    if let Some(end_date) = &params.end_date {
        query = query.filter(audit_log::Column::CreatedAt.lte(end_date));
    }

    let logs = query
        .order_by_desc(audit_log::Column::CreatedAt)
        .all(&state.db)
        .await?;

    let mut wtr = WriterBuilder::new().from_writer(vec![]);

    wtr.write_record(&["ID", "用户ID", "用户名", "操作", "模块", "方法", "路径", "IP地址", "状态码", "描述", "操作时间"])?;

    for log in &logs {
        wtr.write_record(&[
            log.id.to_string(),
            log.user_id.to_string(),
            log.username.clone(),
            log.action.clone(),
            log.module.clone(),
            log.method.clone(),
            log.path.clone(),
            log.ip_address.clone().unwrap_or_default(),
            log.response_status.to_string(),
            log.description.clone().unwrap_or_default(),
            log.created_at.clone(),
        ])?;
    }

    let data = wtr.into_inner().map_err(|e| AppError::CsvInnerError(e.to_string()))?;
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("audit_logs_{}.csv", timestamp);

    let headers = [
        (CONTENT_TYPE, "text/csv"),
        (CONTENT_DISPOSITION, &format!("attachment; filename=\"{}\"", filename)),
    ];

    Ok((headers, data).into_response())
}
