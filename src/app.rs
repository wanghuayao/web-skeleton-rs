use axum::{middleware, Router};

use crate::{
  core::{auth_middleware, AppState},
  modules::{auth_router, todo_router},
};

pub fn create_router() -> Router<AppState> {
  // create a router that will host both of our new routes once we create them
  let api_router = Router::new()
    .nest("/", todo_router())
    .nest("/", auth_router());

  Router::new()
    .nest("/api/v1", api_router)
    .layer(middleware::from_fn(auth_middleware))
}
