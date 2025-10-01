use axum::{http::StatusCode, response::{IntoResponse, Response}};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("User not found")]
    NotFound,

    #[error("Invalid input: {0}")]
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
            AppError::NotFound => (StatusCode::NOT_FOUND, "User not found").into_response(),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
        }
    }
}
