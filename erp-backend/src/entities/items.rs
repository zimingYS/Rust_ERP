/*
    物料实体模块
*/

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub sku: String,
    pub stock: i32,
    pub description: Option<String>,
    pub unit: Option<String>,
    pub min_stock: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation{}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub sku: String,
    pub stock: i32,
    pub description: Option<String>,
    pub unit: Option<String>,
    pub min_stock: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateItemRequest {
    pub name: String,
    pub sku: String,
    pub stock: i32,
    pub description: Option<String>,
    pub unit: Option<String>,
    pub min_stock: Option<i32>,
}

#[derive(Deserialize)]
pub struct StockAdjustmentRequest {
    pub adjustment: i32,
    pub reason: Option<String>,
}

#[derive(Deserialize)]
pub struct ItemQueryParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
    pub sku: Option<String>,
}

#[derive(Serialize)]
pub struct PaginatedItems {
    pub items: Vec<Model>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

#[derive(Serialize)]
pub struct ItemResponse {
    pub code: u16,
    pub message: String,
    pub data: Option<Model>,
}

#[derive(Serialize)]
pub struct ItemsListResponse {
    pub code: u16,
    pub message: String,
    pub data: PaginatedItems,
}
