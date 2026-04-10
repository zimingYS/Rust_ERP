use axum::{middleware, routing::get, Router};
use std::sync::Arc;
use axum::routing::post;
use crate::handlers::{item_handler, health, user_handler, purchase_handler};
use crate::middleware::auth_middleware;
use crate::state::AppState;

pub fn create_router(state: Arc<AppState>) -> Router{
    // 构建受保护路由
    let protected_routes = Router::new()
        .route("/api/items", get(item_handler::get_items).post(item_handler::create_item))
        .route("/api/items/{id}",
               axum::routing::delete(item_handler::delete_item).put(item_handler::update_item)
        )
        
        .route("/api/purchase-orders",
               get(purchase_handler::get_purchase_orders)
                   .post(purchase_handler::create_purchase_order)
        )
        .route("/api/purchase-orders/{id}",
               axum::routing::get(purchase_handler::get_purchase_order_with_items)
                   .put(purchase_handler::update_purchase_order)
                   .delete(purchase_handler::delete_purchase_order)
        )
        .route("/api/purchase-orders/{id}/approve",
               post(purchase_handler::approve_purchase_order)
        )
        
        .layer(middleware::from_fn(auth_middleware));

    Router::new()
        // 公开部分
        .route("/api/health", get(health::health_check))

        .route("/api/register", post(user_handler::register))
        .route("/api/login", post(user_handler::login))

        .merge(protected_routes)
        .with_state(state)
}