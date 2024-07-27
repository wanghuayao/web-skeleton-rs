use axum::{routing::get, Router};

use crate::core::AppState;

use super::todo_2_handlers::list;

pub fn todo_router() -> Router<AppState> {
  Router::new().route("/todos", get(list))
}
