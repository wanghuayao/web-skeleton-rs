use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
  #[error("Resource not found")]
  NotFound,
  #[error("Internal server error")]
  InternalServerError,
}

#[derive(Serialize)]
struct ErrorResponse {
  message: String,
  error: String,
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    let (status, error, message) = match self {
      AppError::NotFound => {
        // This error is caused by bad user input so don't log it
        (StatusCode::NOT_FOUND, "NotFound".to_owned(), "".to_owned())
      }
      AppError::InternalServerError => {
        // This error is caused by a bug in our code so log it
        tracing::error!("Internal server error: {}", self);
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          "InternalServerError".to_owned(),
          "".to_owned(),
        )
      }
    };

    (status, Json(ErrorResponse { error, message })).into_response()
  }
}

/// Convert a SurrealDB error into an AppError
impl From<surrealdb::Error> for AppError {
  fn from(error: surrealdb::Error) -> Self {
    // print error info to log
    tracing::error!("SurrealDB error: {}", error);
    AppError::InternalServerError
  }
}
