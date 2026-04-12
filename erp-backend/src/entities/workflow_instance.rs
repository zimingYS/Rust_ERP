/*
    工作流实例实体
*/
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "workflow_instances")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub workflow_type: String,
    pub business_id: i32,
    pub business_type: String,
    pub current_step: String,
    pub status: String,
    pub initiator_id: i32,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub completed_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::InitiatorId",
        to = "super::user::Column::Id"
    )]
    Initiator,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Initiator.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}