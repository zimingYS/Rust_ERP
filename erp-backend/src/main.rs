mod state;
mod routes;
mod entities;
mod handlers;
mod auth;
mod middleware;
mod database;
mod audit_middleware;
mod constants;
mod permissions;

use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use crate::state::AppState;
use tracing_subscriber::{EnvFilter};
use tracing_subscriber::fmt::writer::MakeWriterExt;

#[tokio::main]
async fn main() {
    // 初始化日志
    println!("初始化日志...");
    let file_appender = tracing_appender::rolling::daily("logs", "backend.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking.and(std::io::stdout))
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    tracing::info!("日志初始化成功!");

    let db = database::initialize_database().await;

    println!("建立连接中...");
    tracing::info!("建立连接中...");

    // 构造共享状态
    let state = Arc::new(AppState { db });

    //  配置中间件
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    // 加载路由
    let app = routes::create_router(state.clone())
        .layer(cors)
        .layer(axum::middleware::from_fn_with_state(
            state,
            audit_middleware::audit_logger_middleware
        ));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    tracing::info!("服务器启动，监听地址: {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

