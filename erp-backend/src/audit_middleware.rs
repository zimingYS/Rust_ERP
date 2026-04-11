use std::sync::Arc;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use chrono::Local;
use sea_orm::{ActiveModelTrait, Set};
use crate::entities::audit_log;
use crate::state::AppState;

pub async fn audit_logger_middleware(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Response {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    let headers = req.headers().clone();
    let ip_address = headers.get("x-forwarded-for")
        .or_else(|| headers.get("x-real-ip"))
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let user_agent = headers.get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let response = next.run(req).await;

    let status = response.status().as_u16() as i32;

    if path.starts_with("/api/") && !path.contains("/health") {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let (user_id, username) = extract_user_from_path(&path);

        let module = determine_module(&path);
        let action = determine_action(&method, &path);
        let description = generate_description(&action, &module, &path);

        let new_log = audit_log::ActiveModel {
            user_id: Set(user_id.unwrap_or(0)),
            username: Set(username.unwrap_or_else(|| "anonymous".to_string())),
            action: Set(action),
            module: Set(module),
            method: Set(method),
            path: Set(path),
            ip_address: Set(ip_address),
            user_agent: Set(user_agent),
            request_data: Set(None),
            response_status: Set(status),
            description: Set(Some(description)),
            created_at: Set(now),
            ..Default::default()
        };

        let db = state.db.clone();
        tokio::spawn(async move {
            let _ = new_log.insert(&db).await;
        });
    }

    response
}

fn extract_user_from_path(path: &str) -> (Option<i32>, Option<String>) {
    if path.contains("/purchase-orders") ||
       path.contains("/sales-orders") ||
       path.contains("/inventory") ||
       path.contains("/finance") {
        return (Some(1), Some("admin".to_string()));
    }
    (None, None)
}

fn determine_module(path: &str) -> String {
    if path.contains("/purchase-orders") {
        "采购管理".to_string()
    } else if path.contains("/sales-orders") {
        "销售管理".to_string()
    } else if path.contains("/inventory") {
        "库存管理".to_string()
    } else if path.contains("/finance") {
        "财务管理".to_string()
    } else if path.contains("/items") {
        "物料管理".to_string()
    } else if path.contains("/users") || path.contains("/register") || path.contains("/login") {
        "用户管理".to_string()
    } else if path.contains("/audit") {
        "日志审计".to_string()
    } else {
        "其他".to_string()
    }
}

fn determine_action(method: &str, path: &str) -> String {
    match method {
        "GET" => {
            if path.split('/').last().map_or(false, |s| s.parse::<i32>().is_ok()) {
                "查看详情".to_string()
            } else {
                "查询列表".to_string()
            }
        },
        "POST" => {
            if path.contains("/approve") || path.contains("/confirm") {
                "审批操作".to_string()
            } else if path.contains("/complete") {
                "完成操作".to_string()
            } else if path.contains("/cancel") {
                "取消操作".to_string()
            } else if path.contains("/adjust") {
                "调整操作".to_string()
            } else {
                "新建记录".to_string()
            }
        },
        "PUT" => "更新记录".to_string(),
        "DELETE" => "删除记录".to_string(),
        _ => "未知操作".to_string(),
    }
}

fn generate_description(action: &str, module: &str, _path: &str) -> String {
    format!("{} - {}", module, action)
}
