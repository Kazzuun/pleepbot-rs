use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // #[error("Unexpected error: {0}")]
    // InternalError(String),
    #[error("Database error")]
    DatabaseError(#[from] SqlxError),

    #[error("Link not found")]
    LinkNotFound,

    #[error("Link expired")]
    LinkExpired,

    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            // AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::LinkNotFound => StatusCode::NOT_FOUND,
            AppError::LinkExpired => StatusCode::GONE,
            AppError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
        };

        let body = Json(json!({ "status": status.as_u16(), "error": self.to_string() }));
        (status, body).into_response()
    }
}
