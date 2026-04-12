/*
    工作流步骤实体
*/
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "workflow_steps")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub instance_id: i32,
    pub step_name: String,
    pub step_order: i32,
    pub approver_id: Option<i32>,
    pub approver_role_id: Option<i32>,
    pub status: String,
    pub action: Option<String>,
    pub comment: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub approved_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::workflow_instance::Entity",
        from = "Column::InstanceId",
        to = "super::workflow_instance::Column::Id"
    )]
    Instance,

    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::ApproverId",
        to = "super::user::Column::Id"
    )]
    Approver,
}

impl Related<super::workflow_instance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Instance.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Approver.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}