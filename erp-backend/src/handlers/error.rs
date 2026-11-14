use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jsonwebtoken::errors::Error as JWTError;
use sea_orm::DbErr;

#[derive(Debug)]
pub enum AppError {
    DbError(DbErr),
    AuthError(String),
    Unauthorized,
    Forbidden(String),
    NotFound(String),
    BadRequest(String),
    InternalError(String),
    CsvError(csv::Error),
    CsvInnerError(String),
    JsonError(serde_json::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DbError(err) => {
                tracing::error!("数据库错误: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "数据库操作失败".to_string(),
                )
            }
            AppError::AuthError(msg) => {
                tracing::warn!("认证错误: {}", msg);
                (StatusCode::UNAUTHORIZED, msg)
            }
            AppError::Unauthorized => {
                tracing::warn!("未授权访问");
                (StatusCode::UNAUTHORIZED, "未授权访问".to_string())
            }
            AppError::Forbidden(msg) => {
                tracing::warn!("禁止访问: {}", msg);
                (StatusCode::FORBIDDEN, msg)
            }
            AppError::NotFound(msg) => {
                tracing::warn!("资源不存在: {}", msg);
                (StatusCode::NOT_FOUND, msg)
            }
            AppError::BadRequest(msg) => {
                tracing::warn!("请求错误: {}", msg);
                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::InternalError(msg) => {
                tracing::error!("内部错误: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            AppError::CsvError(err) => {
                tracing::error!("CSV导出错误: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "CSV导出失败".to_string())
            }
            AppError::CsvInnerError(msg) => {
                tracing::error!("CSV内部错误: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "CSV处理失败".to_string())
            }
            AppError::JsonError(err) => {
                tracing::error!("JSON序列化错误: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "JSON序列化失败".to_string(),
                )
            }
        };

        let body = serde_json::json!({
            "error": error_message
        });

        (status, axum::Json(body)).into_response()
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        AppError::DbError(err)
    }
}

impl From<JWTError> for AppError {
    fn from(err: JWTError) -> Self {
        AppError::AuthError(format!("Token验证失败: {}", err))
    }
}

impl From<csv::Error> for AppError {
    fn from(err: csv::Error) -> Self {
        AppError::CsvError(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::JsonError(err)
    }
}
