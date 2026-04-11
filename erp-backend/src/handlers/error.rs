use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use tracing::error;

#[derive(Debug,Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    #[allow(dead_code)]
    Unauthorized,
    NotFound(String),
    #[allow(dead_code)]
    BadRequest(String),
    #[allow(dead_code)]
    InternalError(String),
    Database(sea_orm::DbErr),
    Jwt(jsonwebtoken::errors::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::Database(e) => {
                error!("数据库错误: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库错误: {}", e))
            },
            AppError::NotFound(msg) => {
                error!("未找到: {}", msg);
                (StatusCode::NOT_FOUND, msg.clone())
            },
            AppError::Unauthorized => {
                error!("未授权访问");
                (StatusCode::UNAUTHORIZED, "未授权".to_string())
            },
            AppError::BadRequest(msg) => {
                error!("请求错误: {}", msg);
                (StatusCode::BAD_REQUEST, msg.clone())
            },
            AppError::InternalError(msg) => {
                error!("内部错误: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            },
            AppError::Jwt(e) => {
                error!("JWT错误: {:?}", e);
                (StatusCode::UNAUTHORIZED, "令牌验证失败".to_string())
            },
        };
        let body = Json(ErrorResponse {
            code: status.as_u16(),
            message,
        });
        (status, body).into_response()
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(e: sea_orm::DbErr) -> Self {
        AppError::Database(e)
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        AppError::Jwt(e)
    }
}
