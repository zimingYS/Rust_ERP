use axum::{middleware, routing::get, Router};
use std::sync::Arc;
use axum::routing::post;
use crate::handlers::{item_handler, health, user_handler};
use crate::middleware::auth_middleware;
use crate::state::AppState;

pub fn create_router(state: Arc<AppState>) -> Router{
    // 构建受保护路由
    let protected_routes = Router::new()
        .route("/api/items", get(item_handler::get_items).post(item_handler::create_item))
        .route("/api/items/{id}",
               axum::routing::delete(item_handler::delete_item).put(item_handler::update_item)
        )
        .layer(middleware::from_fn(auth_middleware));

    Router::new()
        // 公开部分
        // .route("/api/login", post(login::login))
        .route("/api/health", get(health::health_check))

        .route("/api/register", post(user_handler::register))
        .route("/api/login", post(user_handler::login))

        .merge(protected_routes)
        .with_state(state)
}