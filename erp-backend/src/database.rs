use sea_orm::{Database, ConnectionTrait, EntityTrait, PaginatorTrait, ActiveModelTrait, DatabaseConnection};
use tracing;
use entities::role::{Entity as RoleEntity, ActiveModel as RoleActiveModel};
use crate::entities;

pub async fn connect_db() -> DatabaseConnection {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    println!("数据库连接中...");
    tracing::info!("初始化数据库中...");

    match Database::connect(db_url).await {
        Ok(db) => {
            tracing::info!("数据库连接成功!");
            db
        }
        Err(err) => {
            tracing::error!("数据库连接失败: {:?}", err);
            std::process::exit(1);
        }
    }
}

pub async fn init_tables(db: &DatabaseConnection) {
    use sea_orm::{Schema, DbBackend};

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

    let _ = db.execute(builder.build(
        schema.create_table_from_entity(entities::sales_order::Entity).if_not_exists()
    )).await;

    let _ = db.execute(builder.build(
        schema.create_table_from_entity(entities::sales_order_item::Entity).if_not_exists()
    )).await;

    tracing::info!("数据库表创建完成!");
}

pub async fn init_default_roles(db: &DatabaseConnection) {
    let role_count = RoleEntity::find().count(db).await.unwrap_or(0);

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
            let _ = role.insert(db).await;
        }

        tracing::info!("默认角色插入成功！");
    }
}

pub async fn initialize_database() -> DatabaseConnection {
    let db = connect_db().await;
    init_tables(&db).await;
    init_default_roles(&db).await;
    db
}