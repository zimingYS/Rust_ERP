/*
    工作流引擎处理器
*/
use std::sync::Arc;
use axum::extract::{Path, State, Extension};
use axum::Json;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set, QueryFilter, ColumnTrait};
use serde::{Deserialize, Serialize};
use crate::entities::{workflow_instance, workflow_step, user};
use crate::handlers::error::AppError;
use crate::state::AppState;
use crate::auth::UserClaims;

#[derive(Deserialize)]
pub struct StartWorkflowRequest {
    pub workflow_type: String,
    pub business_id: i32,
    pub business_type: String,
}

#[derive(Serialize)]
pub struct WorkflowInstanceResponse {
    pub id: i32,
    pub workflow_type: String,
    pub business_id: i32,
    pub business_type: String,
    pub current_step: String,
    pub status: String,
    pub initiator_id: i32,
    pub steps: Vec<WorkflowStepResponse>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct WorkflowStepResponse {
    pub id: i32,
    pub step_name: String,
    pub step_order: i32,
    pub approver_id: Option<i32>,
    pub approver_name: Option<String>,
    pub status: String,
    pub action: Option<String>,
    pub comment: Option<String>,
    pub approved_at: Option<String>,
}

#[derive(Deserialize)]
pub struct ApproveRequest {
    pub action: String,
    pub comment: Option<String>,
}

pub async fn start_workflow(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<UserClaims>,
    Json(req): Json<StartWorkflowRequest>,
) -> Result<Json<WorkflowInstanceResponse>, AppError> {
    let now = Utc::now();

    // 创建工作流实例
    let instance = workflow_instance::ActiveModel {
        workflow_type: Set(req.workflow_type.clone()),
        business_id: Set(req.business_id),
        business_type: Set(req.business_type.clone()),
        current_step: Set("待审批".to_string()),
        status: Set("进行中".to_string()),
        initiator_id: Set(claims.user_id),
        created_at: Set(now),
        updated_at: Set(now),
        completed_at: Set(None),
        ..Default::default()
    };

    let instance = instance.insert(&state.db).await?;

    // 根据工作流类型创建步骤
    let steps = create_workflow_steps(&state, instance.id, &req.workflow_type).await?;

    tracing::info!(
        "创建工作流实例: ID={}, 类型={}, 业务ID={}",
        instance.id,
        req.workflow_type,
        req.business_id
    );

    Ok(Json(WorkflowInstanceResponse {
        id: instance.id,
        workflow_type: instance.workflow_type,
        business_id: instance.business_id,
        business_type: instance.business_type,
        current_step: instance.current_step,
        status: instance.status,
        initiator_id: instance.initiator_id,
        steps,
        created_at: instance.created_at.to_rfc3339(),
        updated_at: instance.updated_at.to_rfc3339(),
    }))
}

async fn create_workflow_steps(
    state: &AppState,
    instance_id: i32,
    workflow_type: &str,
) -> Result<Vec<WorkflowStepResponse>, AppError> {
    let now = Utc::now();
    let mut steps = Vec::new();

    match workflow_type {
        "purchase_approval" => {
            // 采购审批流程：主管审批 -> 经理审批
            let step1 = workflow_step::ActiveModel {
                instance_id: Set(instance_id),
                step_name: Set("主管审批".to_string()),
                step_order: Set(1),
                approver_role_id: Set(Some(2)), // 主管角色
                status: Set("待处理".to_string()),
                created_at: Set(now),
                updated_at: Set(now),
                ..Default::default()
            };
            let step1 = step1.insert(&state.db).await?;
            steps.push(convert_step_to_response(state, step1).await?);

            let step2 = workflow_step::ActiveModel {
                instance_id: Set(instance_id),
                step_name: Set("经理审批".to_string()),
                step_order: Set(2),
                approver_role_id: Set(Some(1)), // 管理员/经理角色
                status: Set("等待中".to_string()),
                created_at: Set(now),
                updated_at: Set(now),
                ..Default::default()
            };
            let step2 = step2.insert(&state.db).await?;
            steps.push(convert_step_to_response(state, step2).await?);
        }
        "sales_confirmation" => {
            // 销售确认流程：销售主管确认
            let step1 = workflow_step::ActiveModel {
                instance_id: Set(instance_id),
                step_name: Set("销售确认".to_string()),
                step_order: Set(1),
                approver_role_id: Set(Some(2)),
                status: Set("待处理".to_string()),
                created_at: Set(now),
                updated_at: Set(now),
                ..Default::default()
            };
            let step1 = step1.insert(&state.db).await?;
            steps.push(convert_step_to_response(state, step1).await?);
        }
        _ => {
            return Err(AppError::BadRequest(format!("不支持的工作流类型: {}", workflow_type)));
        }
    }

    Ok(steps)
}

async fn convert_step_to_response(
    state: &AppState,
    step: workflow_step::Model,
) -> Result<WorkflowStepResponse, AppError> {
    let approver_name = if let Some(approver_id) = step.approver_id {
        let user = user::Entity::find_by_id(approver_id)
            .one(&state.db)
            .await?;
        user.map(|u| u.username)
    } else {
        None
    };

    Ok(WorkflowStepResponse {
        id: step.id,
        step_name: step.step_name,
        step_order: step.step_order,
        approver_id: step.approver_id,
        approver_name,
        status: step.status,
        action: step.action,
        comment: step.comment,
        approved_at: step.approved_at.map(|t| t.to_rfc3339()),
    })
}

pub async fn approve_step(
    State(state): State<Arc<AppState>>,
    Path(step_id): Path<i32>,
    Extension(claims): Extension<UserClaims>,
    Json(req): Json<ApproveRequest>,
) -> Result<Json<WorkflowInstanceResponse>, AppError> {
    let now = Utc::now();

    // 获取步骤
    let step = workflow_step::Entity::find_by_id(step_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("步骤 {} 不存在", step_id)))?;

    // 保存 instance_id 供后续使用
    let instance_id = step.instance_id;

    // 验证权限
    if let Some(approver_id) = step.approver_id {
        if approver_id != claims.user_id {
            return Err(AppError::Unauthorized);
        }
    }

    // 更新步骤状态
    let mut step_model: workflow_step::ActiveModel = step.into();
    step_model.status = Set(if req.action == "approve" { "已通过".to_string() } else { "已拒绝".to_string() });
    step_model.action = Set(Some(req.action.clone()));
    step_model.comment = Set(req.comment.clone());
    step_model.approved_at = Set(Some(now));
    step_model.updated_at = Set(now);
    step_model.update(&state.db).await?;

    // 获取工作流实例
    let instance = workflow_instance::Entity::find_by_id(instance_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("工作流实例不存在".to_string()))?;

    // 如果拒绝，直接结束工作流
    if req.action == "reject" {
        let mut instance_model: workflow_instance::ActiveModel = instance.clone().into();
        instance_model.status = Set("已拒绝".to_string());
        instance_model.current_step = Set("已拒绝".to_string());
        instance_model.completed_at = Set(Some(now));
        instance_model.updated_at = Set(now);
        instance_model.update(&state.db).await?;

        // 更新业务状态
        update_business_status(&state, &instance, "rejected").await?;

        return get_workflow_instance(state, instance.id).await;
    }

    // 检查是否还有下一步
    let next_step = workflow_step::Entity::find()
        .filter(workflow_step::Column::InstanceId.eq(instance.id))
        .filter(workflow_step::Column::Status.eq("等待中"))
        .order_by_asc(workflow_step::Column::StepOrder)
        .one(&state.db)
        .await?;

    if let Some(next) = next_step {
        // 激活下一步
        let mut next_model: workflow_step::ActiveModel = next.clone().into();
        next_model.status = Set("待处理".to_string());
        next_model.updated_at = Set(now);
        next_model.update(&state.db).await?;

        // 更新实例当前步骤
        let mut instance_model: workflow_instance::ActiveModel = instance.clone().into();
        instance_model.current_step = Set(next.step_name.clone());
        instance_model.updated_at = Set(now);
        instance_model.update(&state.db).await?;
    } else {
        // 所有步骤完成
        let mut instance_model: workflow_instance::ActiveModel = instance.clone().into();
        instance_model.status = Set("已完成".to_string());
        instance_model.current_step = Set("已完成".to_string());
        instance_model.completed_at = Set(Some(now));
        instance_model.updated_at = Set(now);
        instance_model.update(&state.db).await?;

        // 更新业务状态为已批准
        update_business_status(&state, &instance, "approved").await?;
    }

    get_workflow_instance(state, instance.id).await
}

async fn update_business_status(
    state: &AppState,
    instance: &workflow_instance::Model,
    action: &str,
) -> Result<(), AppError> {
    use crate::entities::{purchase_order, sales_order};

    match instance.business_type.as_str() {
        "purchase_order" => {
            if let Some(order) = purchase_order::Entity::find_by_id(instance.business_id)
                .one(&state.db)
                .await?
            {
                let mut order_model: purchase_order::ActiveModel = order.into();
                order_model.status = Set(if action == "approved" { "已批准".to_string() } else { "已拒绝".to_string() });
                order_model.update(&state.db).await?;
            }
        }
        "sales_order" => {
            if let Some(order) = sales_order::Entity::find_by_id(instance.business_id)
                .one(&state.db)
                .await?
            {
                let mut order_model: sales_order::ActiveModel = order.into();
                order_model.status = Set(if action == "approved" { "已确认".to_string() } else { "已拒绝".to_string() });
                order_model.update(&state.db).await?;
            }
        }
        _ => {}
    }

    Ok(())
}

async fn get_workflow_instance(
    state: Arc<AppState>,
    instance_id: i32,
) -> Result<Json<WorkflowInstanceResponse>, AppError> {
    let instance = workflow_instance::Entity::find_by_id(instance_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("工作流实例 {} 不存在", instance_id)))?;

    let steps = workflow_step::Entity::find()
        .filter(workflow_step::Column::InstanceId.eq(instance_id))
        .order_by_asc(workflow_step::Column::StepOrder)
        .all(&state.db)
        .await?;

    let mut step_responses = Vec::new();
    for step in steps {
        step_responses.push(convert_step_to_response(&state, step).await?);
    }

    Ok(Json(WorkflowInstanceResponse {
        id: instance.id,
        workflow_type: instance.workflow_type,
        business_id: instance.business_id,
        business_type: instance.business_type,
        current_step: instance.current_step,
        status: instance.status,
        initiator_id: instance.initiator_id,
        steps: step_responses,
        created_at: instance.created_at.to_rfc3339(),
        updated_at: instance.updated_at.to_rfc3339(),
    }))
}

pub async fn get_my_pending_approvals(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<UserClaims>,
) -> Result<Json<Vec<WorkflowInstanceResponse>>, AppError> {
    // 获取用户待审批的步骤
    let pending_steps = workflow_step::Entity::find()
        .filter(workflow_step::Column::Status.eq("待处理"))
        .filter(
            workflow_step::Column::ApproverId.eq(claims.user_id)
                .or(workflow_step::Column::ApproverRoleId.is_null())
        )
        .all(&state.db)
        .await?;

    let mut instances = Vec::new();
    let mut seen_instances = std::collections::HashSet::new();

    for step in pending_steps {
        if !seen_instances.contains(&step.instance_id) {
            seen_instances.insert(step.instance_id);
            let instance = get_workflow_instance(state.clone(), step.instance_id).await?;
            instances.push(instance.0);
        }
    }

    Ok(Json(instances))
}

pub async fn get_workflow_history(
    State(state): State<Arc<AppState>>,
    Path(business_type): Path<String>,
    Path(business_id): Path<i32>,
) -> Result<Json<Option<WorkflowInstanceResponse>>, AppError> {
    let instance = workflow_instance::Entity::find()
        .filter(workflow_instance::Column::BusinessType.eq(business_type))
        .filter(workflow_instance::Column::BusinessId.eq(business_id))
        .one(&state.db)
        .await?;

    if let Some(inst) = instance {
        let response = get_workflow_instance(state, inst.id).await?;
        Ok(Json(Some(response.0)))
    } else {
        Ok(Json(None))
    }
}