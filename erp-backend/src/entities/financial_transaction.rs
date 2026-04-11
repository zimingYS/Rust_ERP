/*
    财务交易实体模块
*/

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "financial_transactions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub transaction_type: String,
    pub category: String,
    pub amount: f64,
    pub description: Option<String>,
    pub transaction_date: String,
    pub related_order_id: Option<i32>,
    pub related_order_type: Option<String>,
    pub created_by: i32,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::CreatedBy",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
