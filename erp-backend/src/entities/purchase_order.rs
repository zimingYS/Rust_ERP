/*
    采购订单实体模块
*/

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "purchase_orders")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub id: i32,
    pub order_number: String,
    pub supplier: String,
    pub order_date: String,
    pub expected_delivery_date: String,
    pub status: String,
    pub total_amount: f64,
    pub created_by: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}