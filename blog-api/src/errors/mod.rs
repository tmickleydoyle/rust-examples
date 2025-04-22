use axum::{response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFoundError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match &self {
            AppError::ValidationError(_) => (axum::http::StatusCode::BAD_REQUEST, self.to_string()),
            AppError::NotFoundError(_) => (axum::http::StatusCode::NOT_FOUND, self.to_string()),
            AppError::DatabaseError(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            AppError::InternalError(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            AppError::BadRequest(_) => (axum::http::StatusCode::BAD_REQUEST, self.to_string()),
        };

        let body = Json(json!({
            "error": {
                "message": err_msg,
                "code": status.as_u16()
            }
        }));

        (status, body).into_response()
    }
}

// Allow conversion from anyhow::Error to AppError
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}

// Define our app Result type
pub type Result<T> = std::result::Result<T, AppError>;