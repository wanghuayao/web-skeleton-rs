use axum::{middleware, routing::get, Router};

use crate::{
  core::{auth_middleware, health_check, AppState},
  modules::{auth_router, todo_router},
  repositories::connect_surreal_db,
};

pub async fn init() {
  let _ = connect_surreal_db().await;
}

pub fn create_router() -> Router<AppState> {
  // create a router that will host both of our new routes once we create them
  let api_router = Router::new()
    .nest("/", todo_router())
    .nest("/", auth_router());

  Router::new()
    .route("/health", get(health_check))
    .nest("/api/v1", api_router)
    .layer(middleware::from_fn(auth_middleware))
}
