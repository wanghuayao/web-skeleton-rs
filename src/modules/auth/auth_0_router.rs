use axum::{routing::post, Router};

use super::auth_2_handlers::{login, register};

pub fn auth_router() -> Router {
  Router::new()
    .route("/auth/login", post(login))
    .route("/auth/register", post(register))
}
