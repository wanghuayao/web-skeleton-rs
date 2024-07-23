use axum::{routing::get, Router};

use super::todo_2_handlers::list;

pub fn todo_router() -> Router {
  Router::new().route("/todos", get(list))
}
