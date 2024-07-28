use axum::{extract::State, Json};

use crate::core::{AppError, AppState};

use super::todo_1_models::Todo;

pub async fn list(
  State(AppState { config: _ }): State<AppState>,
) -> Result<Json<Vec<Todo>>, AppError> {
  // println!("list config: {:?}", config);

  Err(AppError::NotFound)

  // let json_response = serde_json::json!({
  //     "status": "success".to_string(),
  //     "results": 1,
  //     "todos": vec!["a"],
  // });

  // Ok(Json(json_response))
}
