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
    };

    (status, Json(ErrorResponse { error, message })).into_response()
  }
}
