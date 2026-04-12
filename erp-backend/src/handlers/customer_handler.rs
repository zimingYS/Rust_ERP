/*
    客户管理接口
*/
use std::sync::Arc;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set, QueryFilter, ColumnTrait};
use serde::{Deserialize, Serialize};
use crate::entities::customer;
use crate::handlers::error::AppError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateCustomerRequest {
    pub name: String,
    pub contact_person: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub tax_number: Option<String>,
    pub bank_account: Option<String>,
    pub credit_limit: Option<Decimal>,
    pub rating: Option<i32>,
    pub level: Option<String>,
    pub remark: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateCustomerRequest {
    pub name: String,
    pub contact_person: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub tax_number: Option<String>,
    pub bank_account: Option<String>,
    pub credit_limit: Option<Decimal>,
    pub rating: Option<i32>,
    pub level: Option<String>,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Deserialize)]
pub struct CustomerQueryParams {
    pub keyword: Option<String>,
    pub status: Option<String>,
    pub level: Option<String>,
}

#[derive(Serialize)]
pub struct CustomerStats {
    pub total_customers: u64,
    pub active_customers: u64,
    pub inactive_customers: u64,
    pub avg_rating: f64,
}

pub async fn get_customers(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CustomerQueryParams>,
) -> Result<Json<Vec<customer::Model>>, AppError> {
    let mut query = customer::Entity::find();

    if let Some(keyword) = &params.keyword {
        query = query.filter(
            customer::Column::Name.contains(keyword)
                .or(customer::Column::ContactPerson.contains(keyword))
                .or(customer::Column::Phone.contains(keyword))
        );
    }

    if let Some(status) = &params.status {
        query = query.filter(customer::Column::Status.eq(status));
    }

    if let Some(level) = &params.level {
        query = query.filter(customer::Column::Level.eq(level));
    }

    let customers = query
        .order_by_desc(customer::Column::CreatedAt)
        .all(&state.db)
        .await?;

    Ok(Json(customers))
}

pub async fn get_customer_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<CustomerStats>, AppError> {
    let all_customers = customer::Entity::find().all(&state.db).await?;

    let total_customers = all_customers.len() as u64;
    let active_customers = all_customers.iter().filter(|c| c.status == "启用").count() as u64;
    let inactive_customers = all_customers.iter().filter(|c| c.status == "停用").count() as u64;

    let ratings: Vec<f64> = all_customers.iter()
        .filter_map(|c| c.rating.map(|r| r as f64))
        .collect();

    let avg_rating = if ratings.is_empty() {
        0.0
    } else {
        ratings.iter().sum::<f64>() / ratings.len() as f64
    };

    Ok(Json(CustomerStats {
        total_customers,
        active_customers,
        inactive_customers,
        avg_rating,
    }))
}

pub async fn get_customer_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<customer::Model>, AppError> {
    let customer = customer::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("客户 {} 不存在", id)))?;

    Ok(Json(customer))
}

pub async fn create_customer(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCustomerRequest>,
) -> Result<(StatusCode, Json<customer::Model>), AppError> {
    if payload.name.is_empty() {
        return Err(AppError::BadRequest("客户名称不能为空".to_string()));
    }

    let now = Utc::now();
    let customer_code = format!("CUS{}", Utc::now().format("%Y%m%d%H%M%S"));

    let new_customer = customer::ActiveModel {
        customer_code: Set(customer_code),
        name: Set(payload.name),
        contact_person: Set(payload.contact_person),
        phone: Set(payload.phone),
        email: Set(payload.email),
        address: Set(payload.address),
        tax_number: Set(payload.tax_number),
        bank_account: Set(payload.bank_account),
        credit_limit: Set(payload.credit_limit),
        rating: Set(payload.rating.or(Some(3))),
        level: Set(payload.level.or(Some("普通".to_string()))),
        status: Set("启用".to_string()),
        remark: Set(payload.remark),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let inserted_customer = new_customer.insert(&state.db).await?;

    tracing::info!("创建客户成功: {}", inserted_customer.name);
    Ok((StatusCode::CREATED, Json(inserted_customer)))
}

pub async fn update_customer(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCustomerRequest>,
) -> Result<StatusCode, AppError> {
    let customer_model = customer::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("客户 {} 不存在", id)))?;

    let mut customer: customer::ActiveModel = customer_model.into();

    customer.name = Set(payload.name);
    customer.contact_person = Set(payload.contact_person);
    customer.phone = Set(payload.phone);
    customer.email = Set(payload.email);
    customer.address = Set(payload.address);
    customer.tax_number = Set(payload.tax_number);
    customer.bank_account = Set(payload.bank_account);
    customer.credit_limit = Set(payload.credit_limit);
    customer.rating = Set(payload.rating);
    customer.level = Set(payload.level);
    customer.status = Set(payload.status);
    customer.remark = Set(payload.remark);
    customer.updated_at = Set(Utc::now());

    customer.update(&state.db).await?;

    tracing::info!("更新客户成功: ID={}", id);
    Ok(StatusCode::OK)
}

pub async fn delete_customer(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let customer_model = customer::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("客户 {} 不存在", id)))?;

    let res = customer::Entity::delete_by_id(id)
        .exec(&state.db)
        .await?;

    if res.rows_affected > 0 {
        tracing::info!("删除客户成功: ID={}, 名称={}", id, customer_model.name);
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound(format!("客户 {} 不存在", id)))
    }
}