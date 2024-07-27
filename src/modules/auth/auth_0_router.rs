use axum::{routing::post, Router};

use crate::core::AppState;

use super::auth_2_handlers::{login, register};

pub fn auth_router() -> Router<AppState> {
  Router::new()
    .route("/auth/login", post(login))
    .route("/auth/register", post(register))
}
