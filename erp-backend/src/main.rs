mod state;
mod routes;
mod entities;
mod handlers;
mod auth;
mod middleware;

use std::net::SocketAddr;
use std::sync::Arc;
use sea_orm::{ConnectionTrait, Database, Schema};
use tower_http::cors::{Any, CorsLayer};
use crate::state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // 初始化数据库
    let db = Database::connect(db_url).await.expect("Failed to connect to DB");

    // *自动建表逻辑
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let _ = db.execute(builder.build(
        schema.create_table_from_entity(entities::items::Entity).if_not_exists()
    )).await;

    // 构造共享状态
    let state = Arc::new(AppState { db });

    //  配置中间件
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    // 加载路由
    let app = routes::create_router(state).layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

