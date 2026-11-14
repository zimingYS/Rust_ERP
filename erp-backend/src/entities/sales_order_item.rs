/*
    销售订单项实体模块
*/

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sales_order_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub sales_order_id: i32,
    pub item_id: Option<i32>,
    pub item_name: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub total_price: f64,
    pub remark: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sales_order::Entity",
        from = "Column::SalesOrderId",
        to = "super::sales_order::Column::Id"
    )]
    SalesOrder,

    #[sea_orm(
        belongs_to = "super::items::Entity",
        from = "Column::ItemId",
        to = "super::items::Column::Id"
    )]
    Item,
}

impl Related<super::sales_order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SalesOrder.def()
    }
}

impl Related<super::items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Item.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
