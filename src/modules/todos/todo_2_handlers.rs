use axum::Json;

use crate::core::AppError;

use super::todo_1_models::Todo;

pub async fn list() -> Result<Json<Vec<Todo>>, AppError> {
  Err(AppError::NotFound)

  // let json_response = serde_json::json!({
  //     "status": "success".to_string(),
  //     "results": 1,
  //     "todos": vec!["a"],
  // });

  // Ok(Json(json_response))
}
