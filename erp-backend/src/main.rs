mod state;
mod routes;
mod entities;
mod handlers;
mod auth;
mod middleware;

use std::net::SocketAddr;
use std::sync::Arc;
use sea_orm::{EntityTrait, Database, Schema, ConnectionTrait, PaginatorTrait, ActiveModelTrait};
use tower_http::cors::{Any, CorsLayer};
use crate::state::AppState;
use tracing_subscriber::{EnvFilter};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use entities::role::{Entity as RoleEntity, ActiveModel as RoleActiveModel};

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

    println!("数据库连接中...");
    tracing::info!("初始化数据库中...");

    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // 初始化数据库
    let db = match Database::connect(db_url).await {
        Ok(v) => {
            tracing::info!("数据库连接成功!");
            v
        }
        Err(err) => {
            tracing::error!("数据库连接失败: {:?}", err);
            std::process::exit(1);
        }
    };

    // 自动建表
    println!("数据库建表中...");
    tracing::info!("数据库建表中...");
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let _ = db.execute(builder.build(
        schema.create_table_from_entity(entities::role::Entity).if_not_exists()
    )).await;
    let _ = db.execute(builder.build(
        schema.create_table_from_entity(entities::user::Entity).if_not_exists()
    )).await;
    let _ = db.execute(builder.build(
        schema.create_table_from_entity(entities::items::Entity).if_not_exists()
    )).await;
    let _ = db.execute(builder.build(
        schema.create_table_from_entity(entities::purchase_order::Entity).if_not_exists()
    )).await;
    let _ = db.execute(builder.build(
        schema.create_table_from_entity(entities::purchase_order_item::Entity).if_not_exists()
    )).await;

    let role_count = RoleEntity::find().count(&db).await.unwrap_or(0);
    if role_count == 0 {
        tracing::info!("插入默认角色...");

        let default_roles = vec![
            RoleActiveModel {
                id: sea_orm::ActiveValue::Set(1),
                name: sea_orm::ActiveValue::Set("管理员".to_string()),
            },
            RoleActiveModel {
                id: sea_orm::ActiveValue::Set(2),
                name: sea_orm::ActiveValue::Set("普通用户".to_string()),
            },
        ];

        for role in default_roles {
            let _ = role.insert(&db).await;
        }

        tracing::info!("默认角色插入成功！");
    }

    println!("建立连接中...");
    tracing::info!("建立连接中...");

    // 构造共享状态
    let state = Arc::new(AppState { db });

    //  配置中间件
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    // 加载路由
    let app = routes::create_router(state).layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    tracing::info!("服务器启动，监听地址: {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

