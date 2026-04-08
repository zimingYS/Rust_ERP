use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set};
use serde::Deserialize;
use crate::entities::items::{ActiveModel, Column, Entity, Model};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateItemRequest{
    name:String,
    sku:String,
    stock:i32,
}

#[derive(Deserialize)]
pub struct UpdateItemRequest{
    name: String,
    sku: String,
    stock: i32,
}

pub async fn get_items(State(state): State<Arc<AppState>>) -> Json<Vec<Model>> {
    let items = Entity::find()
        .order_by_asc(Column::Id)
        .all(&state.db)
        .await
        .expect("查询数据库失败");
    Json(items)
}

// 新增物料
pub async fn create_item(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateItemRequest>,
) -> (StatusCode, Json<Model>) {
    let new_item = ActiveModel{
        name: Set(payload.name),
        sku: Set(payload.sku),
        stock: Set(payload.stock),
        ..Default::default()
    };

    let inserted_item = new_item
        .insert(&state.db).await.expect("无法插入到数据库！");

    (StatusCode::CREATED , Json(inserted_item))
}

// 删除物料
pub async fn delete_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
)-> StatusCode {
    // 使用 SeaORM 内置删除
    let res = Entity::delete_by_id(id)
        .exec(&state.db)
        .await
        .expect("数据库删除失败");

    if res.rows_affected > 0 {
        StatusCode::NO_CONTENT // 204 成功但无内容返回
    } else {
        StatusCode::NOT_FOUND // 404 ID 不存在
    }
}

// 更新物料
pub async fn update_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateItemRequest>,
) -> StatusCode {
    let item = Entity::find_by_id(id).one(&state.db).await.expect("查询失败");

    if let Some(item) = item {
        let mut item: ActiveModel = item.into();
        item.name = Set(payload.name);
        item.sku = Set(payload.sku);
        item.stock = Set(payload.stock);

        match item.update(&state.db).await {
            Ok(_) => StatusCode::OK,
            Err(e) => {
                eprintln!("更新数据库时出错: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }else {
        StatusCode::NOT_FOUND
    }
}