/*
    数据导入处理器
*/
use crate::auth::UserClaims;
use crate::entities::{items, supplier};
use crate::handlers::error::AppError;
use crate::state::AppState;
use axum::Json;
use axum::extract::{Extension, Multipart, State};
use axum::http::StatusCode;
use calamine::{DataType, Reader, Sheets, open_workbook_auto};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct ImportResult {
    pub success_count: u32,
    pub failed_count: u32,
    pub errors: Vec<String>,
}

pub async fn import_products(
    State(state): State<Arc<AppState>>,
    Extension(_claims): Extension<UserClaims>,
    mut multipart: Multipart,
) -> Result<Json<ImportResult>, AppError> {
    let mut result = ImportResult {
        success_count: 0,
        failed_count: 0,
        errors: Vec::new(),
    };

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?
    {
        let file_name = field.file_name().unwrap_or_default().to_string();
        let data = field
            .bytes()
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        if file_name.is_empty() {
            continue;
        }

        // 根据文件类型解析
        if file_name.ends_with(".csv") {
            match parse_csv_and_import(&state, &data, &mut result).await {
                Ok(_) => {}
                Err(e) => {
                    result
                        .errors
                        .push(format!("文件 {} 解析失败: {}", file_name, e));
                }
            }
        } else if file_name.ends_with(".xlsx") || file_name.ends_with(".xls") {
            match parse_excel_and_import(&state, &data, &mut result).await {
                Ok(_) => {}
                Err(e) => {
                    result
                        .errors
                        .push(format!("文件 {} 解析失败: {}", file_name, e));
                }
            }
        } else {
            result
                .errors
                .push(format!("不支持的文件格式: {}", file_name));
        }
    }

    tracing::info!(
        "产品导入完成: 成功={}, 失败={}",
        result.success_count,
        result.failed_count
    );

    Ok(Json(result))
}

async fn parse_csv_and_import(
    state: &AppState,
    data: &[u8],
    result: &mut ImportResult,
) -> Result<(), String> {
    let csv_data = String::from_utf8_lossy(data);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_data.as_bytes());

    for row_result in rdr.records() {
        match row_result {
            Ok(record) => {
                if record.len() < 3 {
                    result.failed_count += 1;
                    result.errors.push("CSV行数据不完整".to_string());
                    continue;
                }

                let name = record.get(0).unwrap_or("").trim().to_string();
                let sku = record.get(1).unwrap_or("").trim().to_string();
                let stock: i32 = record.get(2).unwrap_or("0").trim().parse().unwrap_or(0);
                let unit = record.get(3).unwrap_or("").trim().to_string();
                let min_stock: Option<i32> = record.get(4).and_then(|v| v.trim().parse().ok());
                let description = record.get(5).unwrap_or("").trim().to_string();

                if name.is_empty() || sku.is_empty() {
                    result.failed_count += 1;
                    result.errors.push("产品名称和SKU不能为空".to_string());
                    continue;
                }

                // 检查SKU是否已存在
                let existing = items::Entity::find()
                    .filter(items::Column::Sku.eq(&sku))
                    .one(&state.db)
                    .await
                    .map_err(|e| format!("数据库查询失败: {}", e))?;

                if existing.is_some() {
                    result.failed_count += 1;
                    result.errors.push(format!("SKU {} 已存在", sku));
                    continue;
                }

                let item = items::ActiveModel {
                    name: Set(name),
                    sku: Set(sku),
                    stock: Set(stock),
                    unit: Set(Some(if unit.is_empty() {
                        "个".to_string()
                    } else {
                        unit
                    })),
                    min_stock: Set(min_stock),
                    description: Set(if description.is_empty() {
                        None
                    } else {
                        Some(description)
                    }),
                    ..Default::default()
                };

                match item.insert(&state.db).await {
                    Ok(_) => result.success_count += 1,
                    Err(e) => {
                        result.failed_count += 1;
                        result.errors.push(format!("插入失败: {}", e));
                    }
                }
            }
            Err(e) => {
                result.failed_count += 1;
                result.errors.push(format!("CSV解析错误: {}", e));
            }
        }
    }

    Ok(())
}

async fn parse_excel_and_import(
    state: &AppState,
    data: &[u8],
    result: &mut ImportResult,
) -> Result<(), String> {
    // 将数据写入临时文件
    let temp_path = format!("/tmp/import_{}.xlsx", Utc::now().timestamp());
    std::fs::write(&temp_path, data).map_err(|e| format!("写入临时文件失败: {}", e))?;

    let mut workbook: Sheets<_> =
        open_workbook_auto(&temp_path).map_err(|e| format!("Excel解析失败: {}", e))?;

    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
        for (row_idx, row) in range.rows().enumerate() {
            // 跳过表头
            if row_idx == 0 {
                continue;
            }

            if row.len() < 3 {
                result.failed_count += 1;
                result.errors.push(format!("第{}行数据不完整", row_idx + 1));
                continue;
            }

            let name = row[0].to_string();
            let sku = row[1].to_string();
            let stock = row[2].as_f64().unwrap_or(0.0) as i32;
            let unit = if row.len() > 3 {
                row[3].to_string()
            } else {
                "".to_string()
            };
            let min_stock = if row.len() > 4 {
                row[4].as_f64().map(|v| v as i32)
            } else {
                None
            };
            let description = if row.len() > 5 {
                row[5].to_string()
            } else {
                "".to_string()
            };

            if name.is_empty() || sku.is_empty() {
                result.failed_count += 1;
                result
                    .errors
                    .push(format!("第{}行: 产品名称和SKU不能为空", row_idx + 1));
                continue;
            }

            // 检查SKU是否已存在
            let existing = items::Entity::find()
                .filter(items::Column::Sku.eq(&sku))
                .one(&state.db)
                .await
                .map_err(|e| format!("数据库查询失败: {}", e))?;

            if existing.is_some() {
                result.failed_count += 1;
                result
                    .errors
                    .push(format!("第{}行: SKU {} 已存在", row_idx + 1, sku));
                continue;
            }

            let item = items::ActiveModel {
                name: Set(name),
                sku: Set(sku),
                stock: Set(stock),
                unit: Set(Some(if unit.is_empty() {
                    "个".to_string()
                } else {
                    unit
                })),
                min_stock: Set(min_stock),
                description: Set(if description.is_empty() {
                    None
                } else {
                    Some(description)
                }),
                ..Default::default()
            };

            match item.insert(&state.db).await {
                Ok(_) => result.success_count += 1,
                Err(e) => {
                    result.failed_count += 1;
                    result
                        .errors
                        .push(format!("第{}行: 插入失败: {}", row_idx + 1, e));
                }
            }
        }
    }

    // 删除临时文件
    let _ = std::fs::remove_file(&temp_path);

    Ok(())
}

pub async fn import_suppliers(
    State(state): State<Arc<AppState>>,
    Extension(_claims): Extension<UserClaims>,
    mut multipart: Multipart,
) -> Result<Json<ImportResult>, AppError> {
    let mut result = ImportResult {
        success_count: 0,
        failed_count: 0,
        errors: Vec::new(),
    };

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?
    {
        let file_name = field.file_name().unwrap_or_default().to_string();
        let data = field
            .bytes()
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        if file_name.is_empty() {
            continue;
        }

        if file_name.ends_with(".csv") {
            match parse_supplier_csv(&state, &data, &mut result).await {
                Ok(_) => {}
                Err(e) => {
                    result
                        .errors
                        .push(format!("文件 {} 解析失败: {}", file_name, e));
                }
            }
        }
    }

    Ok(Json(result))
}

async fn parse_supplier_csv(
    state: &AppState,
    data: &[u8],
    result: &mut ImportResult,
) -> Result<(), String> {
    let csv_data = String::from_utf8_lossy(data);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_data.as_bytes());

    for row_result in rdr.records() {
        match row_result {
            Ok(record) => {
                if record.len() < 2 {
                    result.failed_count += 1;
                    result.errors.push("供应商数据不完整".to_string());
                    continue;
                }

                let name = record.get(0).unwrap_or("").trim().to_string();
                let contact = record.get(1).unwrap_or("").trim().to_string();
                let phone = record.get(2).unwrap_or("").trim().to_string();
                let email = record.get(3).unwrap_or("").trim().to_string();
                let address = record.get(4).unwrap_or("").trim().to_string();

                if name.is_empty() {
                    result.failed_count += 1;
                    result.errors.push("供应商名称不能为空".to_string());
                    continue;
                }

                let sup = supplier::ActiveModel {
                    name: Set(name),
                    contact_person: Set(if contact.is_empty() {
                        None
                    } else {
                        Some(contact)
                    }),
                    phone: Set(if phone.is_empty() { None } else { Some(phone) }),
                    email: Set(if email.is_empty() { None } else { Some(email) }),
                    address: Set(if address.is_empty() {
                        None
                    } else {
                        Some(address)
                    }),
                    ..Default::default()
                };

                match sup.insert(&state.db).await {
                    Ok(_) => result.success_count += 1,
                    Err(e) => {
                        result.failed_count += 1;
                        result.errors.push(format!("插入失败: {}", e));
                    }
                }
            }
            Err(e) => {
                result.failed_count += 1;
                result.errors.push(format!("CSV解析错误: {}", e));
            }
        }
    }

    Ok(())
}

pub async fn get_import_template() -> Result<(StatusCode, String), AppError> {
    let template =
        "产品名称,SKU,初始库存,单位,最低库存,描述\n示例产品,SKU001,100,个,10,这是一个示例产品";

    Ok((StatusCode::OK, template.to_string()))
}
