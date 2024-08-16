use axum::{extract::State, Json};

use crate::{
  core::{AppError, AppState},
  repositories::todo_repository,
};

use super::todo_1_models::Todo;

pub async fn list(
  State(AppState { config: _ }): State<AppState>,
) -> Result<Json<Vec<Todo>>, AppError> {
  let todo = todo_repository::get_all().await?; // .map_err(|e| AppError::from_surreal_error(e))?;

  return Ok(Json(
    todo
      .into_iter()
      .map(|todo| Todo {
        id: Some(todo.id.unwrap().tb),
        title: todo.title,
        content: todo.content,
        completed: todo.completed,
        createdAt: todo.created_at,
        updatedAt: todo.updated_at,
      })
      .collect(),
  ));

  // Err(AppError::NotFound)

  // let json_response = serde_json::json!({
  //     "status": "success".to_string(),
  //     "results": 1,
  //     "todos": vec!["a"],
  // });

  // Ok(Json(json_response))
}
