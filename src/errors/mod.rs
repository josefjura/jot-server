use std::env::VarError;

use aide::axum::IntoApiResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use dto::AppErrorDto;
use serde_json::json;
use thiserror::Error;

pub mod dto;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Cannot bind to selected address. Error: {0}")]
    CannotBind(#[from] std::io::Error),

    #[error("Error while starting the server. Error: {0}")]
    CannotServe(std::io::Error),

    #[error("Error while connecting to the database. Error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Missing environment value: {1}")]
    EnvError(#[source] VarError, String),
}

#[derive(Error, Debug)]
pub enum RestError {
    #[error("Resource not found")]
    NotFound,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Database error: {0}")]
    Database(#[from] DbError),
    #[error("Unauthorized: {0}")]
    Authorization(#[from] AuthError),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("OpenApi: {0}")]
    OpenApi(String),
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Error communicating with database: {0}")]
    Unknown(#[from] sqlx::Error),
}

#[derive(Error, Debug, Clone)]
pub enum AuthError {
    #[error("Username or password incorrect")]
    PasswordIncorrect,
    #[error("Username or password incorrect")]
    UserNotFound,
    #[error("Token was not found")]
    TokenNotFound,
    #[error("Token is not valid")]
    TokenInvalid,
    #[error("Error while connecting to the database.")]
    DatabaseError,
}

// Implementation to convert AppError into a Response
impl IntoResponse for RestError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            RestError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(AppErrorDto::new(&self.to_string())),
            ),
            RestError::InvalidInput(_) => (
                StatusCode::BAD_REQUEST,
                Json(AppErrorDto::new(&self.to_string())),
            ),
            RestError::Authorization(AuthError::TokenInvalid)
            | RestError::Authorization(AuthError::TokenNotFound) => (
                StatusCode::FORBIDDEN,
                Json(AppErrorDto::new(&self.to_string())),
            ),
            RestError::Authorization(_) => (
                StatusCode::UNAUTHORIZED,
                Json(AppErrorDto::new(&self.to_string())),
            ),
            RestError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AppErrorDto::new("Internal server error")),
            ),
            RestError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    AppErrorDto::new("Internal server error")
                        .with_details(json!({ "error": self.to_string() })),
                ),
            ),
            RestError::OpenApi(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    AppErrorDto::new("OpenApi error")
                        .with_details(json!({ "error": self.to_string() })),
                ),
            ),
        };

        (status, error_message).into_response()
    }
}

// Type alias for handler results
pub type RestResult<T> = Result<T, RestError>;
