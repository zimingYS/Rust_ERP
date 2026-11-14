/*
    库存管理接口
*/
use crate::auth::UserClaims;
use crate::constants::{
    ADJUSTMENT_TYPE_COUNTING, ADJUSTMENT_TYPE_INBOUND, ADJUSTMENT_TYPE_LOSS,
    ADJUSTMENT_TYPE_OUTBOUND, ADJUSTMENT_TYPE_RETURN, DEFAULT_LEAD_TIME_DAYS, INVENTORY_DAYS_BASE,
    NOTIFICATION_TYPE_INVENTORY_ALERT, NOTIFICATION_TYPE_REORDER_SUGGESTION, PRIORITY_HIGH,
    PRIORITY_LOW, PRIORITY_MEDIUM, REORDER_DAYS_TARGET, REORDER_MULTIPLIER, SERVICE_FACTOR_95,
};
use crate::entities::{inventory_adjustment, items, notification};
use crate::handlers::error::AppError;
use crate::state::AppState;
use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::http::StatusCode;
use chrono::{Local, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct InventoryQueryParams {
    pub keyword: Option<String>,
    pub sku: Option<String>,
    pub low_stock_only: Option<bool>,
}

#[derive(Deserialize)]
pub struct StockAdjustmentRequest {
    pub adjustment_type: String,
    pub quantity: i32,
    pub reason: Option<String>,
}

#[derive(Serialize)]
pub struct InventoryItem {
    pub id: i32,
    pub name: String,
    pub sku: String,
    pub stock: i32,
    pub unit: Option<String>,
    pub min_stock: Option<i32>,
    pub is_low_stock: bool,
}

#[derive(Serialize)]
pub struct InventoryStats {
    pub total_items: u64,
    pub low_stock_items: u64,
    pub out_of_stock_items: u64,
    pub total_stock_value: u64,
}

#[derive(Serialize)]
pub struct LowStockAlert {
    pub item_id: i32,
    pub item_name: String,
    pub sku: String,
    pub current_stock: i32,
    pub min_stock: i32,
    pub shortage: i32,
    pub suggested_order_qty: i32,
}

#[derive(Serialize)]
pub struct ReorderSuggestion {
    pub item_id: i32,
    pub item_name: String,
    pub sku: String,
    pub current_stock: i32,
    pub avg_daily_usage: f64,
    pub lead_time_days: i32,
    pub safety_stock: i32,
    pub reorder_point: i32,
    pub suggested_qty: i32,
    pub priority: String,
}

fn build_inventory_item(item: &items::Model) -> InventoryItem {
    let min_stock = item.min_stock.unwrap_or(0);
    let is_low_stock = item.stock <= min_stock && item.stock > 0;

    InventoryItem {
        id: item.id,
        name: item.name.clone(),
        sku: item.sku.clone(),
        stock: item.stock,
        unit: item.unit.clone(),
        min_stock: item.min_stock,
        is_low_stock,
    }
}

pub async fn get_inventory_list(
    State(state): State<Arc<AppState>>,
    Query(params): Query<InventoryQueryParams>,
) -> Result<Json<Vec<InventoryItem>>, AppError> {
    let mut query = items::Entity::find();

    if let Some(keyword) = &params.keyword {
        query = query.filter(items::Column::Name.contains(keyword));
    }

    if let Some(sku) = &params.sku {
        query = query.filter(items::Column::Sku.eq(sku));
    }

    let all_items = query.all(&state.db).await?;

    let inventory_items: Vec<InventoryItem> = all_items.iter().map(build_inventory_item).collect();

    let filtered_items: Vec<InventoryItem> = if params.low_stock_only == Some(true) {
        inventory_items
            .into_iter()
            .filter(|item| item.is_low_stock || item.stock == 0)
            .collect()
    } else {
        inventory_items
    };

    Ok(Json(filtered_items))
}

pub async fn get_inventory_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<InventoryStats>, AppError> {
    let all_items = items::Entity::find().all(&state.db).await?;

    let total_items = all_items.len() as u64;
    let low_stock_items = all_items
        .iter()
        .filter(|item| {
            let min_stock = item.min_stock.unwrap_or(0);
            item.stock <= min_stock && item.stock > 0
        })
        .count() as u64;

    let out_of_stock_items = all_items.iter().filter(|item| item.stock == 0).count() as u64;
    let total_stock_value = all_items.iter().map(|item| item.stock).sum::<i32>() as u64;

    Ok(Json(InventoryStats {
        total_items,
        low_stock_items,
        out_of_stock_items,
        total_stock_value,
    }))
}

pub async fn get_item_inventory(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<InventoryItem>, AppError> {
    let item = items::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("物料 {} 不存在", id)))?;

    Ok(Json(build_inventory_item(&item)))
}

pub async fn adjust_stock(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<UserClaims>,
    Path(id): Path<i32>,
    Json(payload): Json<StockAdjustmentRequest>,
) -> Result<Json<InventoryItem>, AppError> {
    let item = items::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("物料 {} 不存在", id)))?;

    if payload.quantity <= 0 {
        return Err(AppError::BadRequest("调整数量必须大于0".to_string()));
    }

    let valid_types = vec![
        ADJUSTMENT_TYPE_INBOUND,
        ADJUSTMENT_TYPE_OUTBOUND,
        ADJUSTMENT_TYPE_COUNTING,
        ADJUSTMENT_TYPE_LOSS,
        ADJUSTMENT_TYPE_RETURN,
    ];
    if !valid_types.contains(&payload.adjustment_type.as_str()) {
        return Err(AppError::BadRequest(format!(
            "无效的调整类型: {}",
            payload.adjustment_type
        )));
    }

    let before_stock = item.stock;
    let new_stock = match payload.adjustment_type.as_str() {
        ADJUSTMENT_TYPE_INBOUND | ADJUSTMENT_TYPE_COUNTING | ADJUSTMENT_TYPE_RETURN => {
            before_stock + payload.quantity
        }
        ADJUSTMENT_TYPE_OUTBOUND | ADJUSTMENT_TYPE_LOSS => {
            if before_stock < payload.quantity {
                return Err(AppError::BadRequest(format!(
                    "库存不足，当前库存: {}, 请求出库: {}",
                    before_stock, payload.quantity
                )));
            }
            before_stock - payload.quantity
        }
        _ => return Err(AppError::BadRequest("无效的调整类型".to_string())),
    };

    let mut item_model: items::ActiveModel = item.clone().into();
    item_model.stock = Set(new_stock);

    let updated_item = item_model.update(&state.db).await?;

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let adjustment_type = payload.adjustment_type.clone();
    let quantity = payload.quantity;

    let adjustment_record = inventory_adjustment::ActiveModel {
        item_id: Set(id),
        adjustment_type: Set(payload.adjustment_type),
        quantity: Set(payload.quantity),
        before_stock: Set(before_stock),
        after_stock: Set(new_stock),
        reason: Set(payload.reason),
        created_by: Set(claims.user_id),
        created_at: Set(now),
        ..Default::default()
    };

    adjustment_record.insert(&state.db).await?;

    tracing::info!(
        "库存调整 - 物料ID: {}, 类型: {}, 数量: {}, 调整前: {}, 调整后: {}",
        id,
        adjustment_type,
        quantity,
        before_stock,
        new_stock
    );

    Ok(Json(build_inventory_item(&updated_item)))
}

pub async fn get_adjustment_history(
    State(state): State<Arc<AppState>>,
    Path(item_id): Path<i32>,
) -> Result<Json<Vec<inventory_adjustment::Model>>, AppError> {
    let adjustments = inventory_adjustment::Entity::find()
        .filter(inventory_adjustment::Column::ItemId.eq(item_id))
        .order_by_desc(inventory_adjustment::Column::CreatedAt)
        .all(&state.db)
        .await?;

    Ok(Json(adjustments))
}

#[allow(dead_code)]
pub async fn batch_adjust_stock(
    State(_state): State<Arc<AppState>>,
    Extension(_claims): Extension<UserClaims>,
    Json(_payload): Json<Vec<StockAdjustmentRequest>>,
) -> Result<StatusCode, AppError> {
    Err(AppError::BadRequest("批量调整功能暂未实现".to_string()))
}

pub async fn get_low_stock_alerts(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<LowStockAlert>>, AppError> {
    let all_items = items::Entity::find().all(&state.db).await?;

    let alerts: Vec<LowStockAlert> = all_items
        .iter()
        .filter(|item| {
            let min_stock = item.min_stock.unwrap_or(0);
            item.stock <= min_stock
        })
        .map(|item| {
            let min_stock = item.min_stock.unwrap_or(0);
            let shortage = min_stock - item.stock;
            // 建议订货量 = 最低库存 * 倍数 - 当前库存
            let suggested_order_qty = (min_stock * REORDER_MULTIPLIER) - item.stock;

            LowStockAlert {
                item_id: item.id,
                item_name: item.name.clone(),
                sku: item.sku.clone(),
                current_stock: item.stock,
                min_stock,
                shortage,
                suggested_order_qty: if suggested_order_qty > 0 {
                    suggested_order_qty
                } else {
                    min_stock
                },
            }
        })
        .collect();

    Ok(Json(alerts))
}

pub async fn get_reorder_suggestions(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ReorderSuggestion>>, AppError> {
    let all_items = items::Entity::find().all(&state.db).await?;

    let suggestions: Vec<ReorderSuggestion> = all_items
        .iter()
        .filter(|item| {
            let min_stock = item.min_stock.unwrap_or(0);
            item.stock <= min_stock * 2 // 库存低于2倍最低库存时给出建议
        })
        .map(|item| {
            let min_stock = item.min_stock.unwrap_or(0);
            let current_stock = item.stock;

            // 简化的平均日用量计算（实际应该基于历史销售数据）
            let avg_daily_usage = if min_stock > 0 {
                min_stock as f64 / INVENTORY_DAYS_BASE
            } else {
                0.0
            };

            let lead_time_days = DEFAULT_LEAD_TIME_DAYS;
            let safety_stock = min_stock; // 安全库存 = 最低库存

            // 再订货点 = 平均日用量 * 提前期 + 安全库存
            let reorder_point = (avg_daily_usage * lead_time_days as f64) as i32 + safety_stock;

            // 建议订货量 = (平均日用量 * 目标天数) - 当前库存
            let suggested_qty = ((avg_daily_usage * REORDER_DAYS_TARGET) as i32) - current_stock;

            // 优先级判断
            let priority = if current_stock == 0 {
                "紧急".to_string()
            } else if current_stock <= min_stock {
                "高".to_string()
            } else {
                "中".to_string()
            };

            ReorderSuggestion {
                item_id: item.id,
                item_name: item.name.clone(),
                sku: item.sku.clone(),
                current_stock,
                avg_daily_usage,
                lead_time_days,
                safety_stock,
                reorder_point,
                suggested_qty: if suggested_qty > 0 {
                    suggested_qty
                } else {
                    min_stock * 2
                },
                priority,
            }
        })
        .collect();

    Ok(Json(suggestions))
}

pub async fn check_and_create_alerts(
    State(state): State<Arc<AppState>>,
) -> Result<Json<u32>, AppError> {
    let all_items = items::Entity::find().all(&state.db).await?;
    let mut alert_count = 0u32;

    for item in all_items {
        let min_stock = item.min_stock.unwrap_or(0);

        if item.stock <= min_stock && item.stock > 0 {
            // 低库存预警
            let title = format!("库存预警: {}", item.name);
            let content = format!(
                "物料 {} (SKU: {}) 当前库存为 {}，已低于最低库存 {}，请及时补货。",
                item.name, item.sku, item.stock, min_stock
            );

            create_notification_for_low_stock(&state, &item, &title, &content, min_stock).await?;
            alert_count += 1;
        } else if item.stock == 0 {
            // 缺货预警
            let title = format!("缺货警告: {}", item.name);
            let content = format!(
                "物料 {} (SKU: {}) 已缺货，请立即采购！",
                item.name, item.sku
            );

            create_notification_for_low_stock(&state, &item, &title, &content, min_stock).await?;
            alert_count += 1;
        }
    }

    tracing::info!("库存预警检查完成，生成 {} 条预警", alert_count);
    Ok(Json(alert_count))
}

async fn create_notification_for_low_stock(
    state: &AppState,
    item: &items::Model,
    title: &str,
    content: &str,
    _min_stock: i32,
) -> Result<(), AppError> {
    use crate::constants::{ROLE_ADMIN, ROLE_SUPERVISOR};
    use crate::entities::user;

    let users = user::Entity::find()
        .filter(
            user::Column::RoleId
                .eq(ROLE_ADMIN)
                .or(user::Column::RoleId.eq(ROLE_SUPERVISOR)),
        )
        .all(&state.db)
        .await?;

    for user in users {
        let now = Utc::now();
        let notification = notification::ActiveModel {
            user_id: Set(user.id),
            title: Set(title.to_string()),
            content: Set(content.to_string()),
            notification_type: Set(NOTIFICATION_TYPE_INVENTORY_ALERT.to_string()),
            is_read: Set(false),
            priority: Set(if item.stock == 0 {
                Some(PRIORITY_HIGH.to_string())
            } else {
                Some(PRIORITY_MEDIUM.to_string())
            }),
            related_id: Set(Some(item.id)),
            related_type: Set(Some("inventory".to_string())),
            created_at: Set(now),
            read_at: Set(None),
            ..Default::default()
        };

        notification.insert(&state.db).await?;
    }

    Ok(())
}

pub async fn create_reorder_notifications(
    State(state): State<Arc<AppState>>,
) -> Result<Json<u32>, AppError> {
    let all_items = items::Entity::find().all(&state.db).await?;
    let mut notification_count = 0u32;

    for item in all_items.iter() {
        let min_stock = item.min_stock.unwrap_or(0);

        if item.stock <= min_stock * REORDER_MULTIPLIER && item.stock >= 0 {
            let avg_daily_usage = if min_stock > 0 {
                min_stock as f64 / INVENTORY_DAYS_BASE
            } else {
                0.0
            };

            let lead_time_days = DEFAULT_LEAD_TIME_DAYS;
            let safety_stock = min_stock;
            let reorder_point = (avg_daily_usage * lead_time_days as f64) as i32 + safety_stock;
            let suggested_qty = ((avg_daily_usage * REORDER_DAYS_TARGET) as i32) - item.stock;
            let final_suggested_qty = if suggested_qty > 0 {
                suggested_qty
            } else {
                min_stock * REORDER_MULTIPLIER
            };

            let priority = if item.stock == 0 {
                PRIORITY_HIGH
            } else if item.stock <= min_stock {
                PRIORITY_MEDIUM
            } else {
                PRIORITY_LOW
            };

            let title = format!("补货建议: {}", item.name);
            let content = format!(
                "物料 {} (SKU: {}) 建议补货。当前库存: {}, 再订货点: {}, 建议订货量: {}, 优先级: {}",
                item.name, item.sku, item.stock, reorder_point, final_suggested_qty, priority
            );

            create_notification_for_reorder(&state, &item, &title, &content, priority).await?;
            notification_count += 1;
        }
    }

    tracing::info!("补货建议通知创建完成，生成 {} 条通知", notification_count);
    Ok(Json(notification_count))
}

async fn create_notification_for_reorder(
    state: &AppState,
    item: &items::Model,
    title: &str,
    content: &str,
    priority: &str,
) -> Result<(), AppError> {
    use crate::constants::{ROLE_ADMIN, ROLE_SUPERVISOR};
    use crate::entities::user;

    let users = user::Entity::find()
        .filter(
            user::Column::RoleId
                .eq(ROLE_ADMIN)
                .or(user::Column::RoleId.eq(ROLE_SUPERVISOR)),
        )
        .all(&state.db)
        .await?;

    for user in users {
        let now = Utc::now();
        let notification = notification::ActiveModel {
            user_id: Set(user.id),
            title: Set(title.to_string()),
            content: Set(content.to_string()),
            notification_type: Set(NOTIFICATION_TYPE_REORDER_SUGGESTION.to_string()),
            is_read: Set(false),
            priority: Set(Some(priority.to_string())),
            related_id: Set(Some(item.id)),
            related_type: Set(Some("inventory".to_string())),
            created_at: Set(now),
            read_at: Set(None),
            ..Default::default()
        };

        notification.insert(&state.db).await?;
    }

    Ok(())
}

pub async fn auto_calculate_safety_stock(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<i32>, AppError> {
    let item = items::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("物料 {} 不存在", id)))?;

    // 简化的安全库存计算
    // 实际应该基于历史数据的标准差和服务水平
    let avg_daily_usage = if item.min_stock.is_some() {
        item.min_stock.unwrap() as f64 / INVENTORY_DAYS_BASE
    } else {
        0.0
    };

    let lead_time_days = DEFAULT_LEAD_TIME_DAYS;
    let service_factor = SERVICE_FACTOR_95;

    // 安全库存 = Z值 * 平均日用量 * sqrt(提前期)
    let safety_stock = (service_factor * avg_daily_usage * (lead_time_days as f64).sqrt()) as i32;
    let safety_stock = if safety_stock < 0 { 0 } else { safety_stock };

    Ok(Json(safety_stock))
}
