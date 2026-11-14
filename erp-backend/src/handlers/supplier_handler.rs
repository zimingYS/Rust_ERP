/*
    供应商管理接口
*/
use crate::entities::supplier;
use crate::handlers::error::AppError;
use crate::state::AppState;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use chrono::Utc;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CreateSupplierRequest {
    pub name: String,
    pub contact_person: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub tax_number: Option<String>,
    pub bank_account: Option<String>,
    pub payment_terms: Option<String>,
    pub credit_limit: Option<Decimal>,
    pub rating: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateSupplierRequest {
    pub name: String,
    pub contact_person: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub tax_number: Option<String>,
    pub bank_account: Option<String>,
    pub payment_terms: Option<String>,
    pub credit_limit: Option<Decimal>,
    pub rating: Option<i32>,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Deserialize)]
pub struct SupplierQueryParams {
    pub keyword: Option<String>,
    pub status: Option<String>,
    pub rating: Option<i32>,
}

#[derive(Serialize)]
pub struct SupplierStats {
    pub total_suppliers: u64,
    pub active_suppliers: u64,
    pub inactive_suppliers: u64,
    pub avg_rating: f64,
}

pub async fn get_suppliers(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SupplierQueryParams>,
) -> Result<Json<Vec<supplier::Model>>, AppError> {
    let mut query = supplier::Entity::find();

    if let Some(keyword) = &params.keyword {
        query = query.filter(
            supplier::Column::Name
                .contains(keyword)
                .or(supplier::Column::ContactPerson.contains(keyword))
                .or(supplier::Column::Phone.contains(keyword)),
        );
    }

    if let Some(status) = &params.status {
        query = query.filter(supplier::Column::Status.eq(status));
    }

    if let Some(rating) = params.rating {
        query = query.filter(supplier::Column::Rating.eq(rating));
    }

    let suppliers = query
        .order_by_desc(supplier::Column::CreatedAt)
        .all(&state.db)
        .await?;

    Ok(Json(suppliers))
}

pub async fn get_supplier_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<SupplierStats>, AppError> {
    let all_suppliers = supplier::Entity::find().all(&state.db).await?;

    let total_suppliers = all_suppliers.len() as u64;
    let active_suppliers = all_suppliers.iter().filter(|s| s.status == "启用").count() as u64;
    let inactive_suppliers = all_suppliers.iter().filter(|s| s.status == "停用").count() as u64;

    let ratings: Vec<f64> = all_suppliers
        .iter()
        .filter_map(|s| s.rating.map(|r| r as f64))
        .collect();

    let avg_rating = if ratings.is_empty() {
        0.0
    } else {
        ratings.iter().sum::<f64>() / ratings.len() as f64
    };

    Ok(Json(SupplierStats {
        total_suppliers,
        active_suppliers,
        inactive_suppliers,
        avg_rating,
    }))
}

pub async fn get_supplier_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<supplier::Model>, AppError> {
    let supplier = supplier::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("供应商 {} 不存在", id)))?;

    Ok(Json(supplier))
}

pub async fn create_supplier(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateSupplierRequest>,
) -> Result<(StatusCode, Json<supplier::Model>), AppError> {
    if payload.name.is_empty() {
        return Err(AppError::BadRequest("供应商名称不能为空".to_string()));
    }

    let now = Utc::now();
    let supplier_code = format!("SUP{}", Utc::now().format("%Y%m%d%H%M%S"));

    let new_supplier = supplier::ActiveModel {
        supplier_code: Set(supplier_code),
        name: Set(payload.name),
        contact_person: Set(payload.contact_person),
        phone: Set(payload.phone),
        email: Set(payload.email),
        address: Set(payload.address),
        tax_number: Set(payload.tax_number),
        bank_account: Set(payload.bank_account),
        payment_terms: Set(payload.payment_terms),
        credit_limit: Set(payload.credit_limit),
        rating: Set(payload.rating.or(Some(3))),
        status: Set("启用".to_string()),
        remark: Set(payload.remark),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let inserted_supplier = new_supplier.insert(&state.db).await?;

    tracing::info!("创建供应商成功: {}", inserted_supplier.name);
    Ok((StatusCode::CREATED, Json(inserted_supplier)))
}

pub async fn update_supplier(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateSupplierRequest>,
) -> Result<StatusCode, AppError> {
    let supplier_model = supplier::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("供应商 {} 不存在", id)))?;

    let mut supplier: supplier::ActiveModel = supplier_model.into();

    supplier.name = Set(payload.name);
    supplier.contact_person = Set(payload.contact_person);
    supplier.phone = Set(payload.phone);
    supplier.email = Set(payload.email);
    supplier.address = Set(payload.address);
    supplier.tax_number = Set(payload.tax_number);
    supplier.bank_account = Set(payload.bank_account);
    supplier.payment_terms = Set(payload.payment_terms);
    supplier.credit_limit = Set(payload.credit_limit);
    supplier.rating = Set(payload.rating);
    supplier.status = Set(payload.status);
    supplier.remark = Set(payload.remark);
    supplier.updated_at = Set(Utc::now());

    supplier.update(&state.db).await?;

    tracing::info!("更新供应商成功: ID={}", id);
    Ok(StatusCode::OK)
}

pub async fn delete_supplier(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let supplier_model = supplier::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("供应商 {} 不存在", id)))?;

    let res = supplier::Entity::delete_by_id(id).exec(&state.db).await?;

    if res.rows_affected > 0 {
        tracing::info!("删除供应商成功: ID={}, 名称={}", id, supplier_model.name);
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound(format!("供应商 {} 不存在", id)))
    }
}
