use std::sync::Arc;

use axum::{
  http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
  },
  Router,
};

use tower_http::cors::CorsLayer;
use web_skeleton::{
  app::create_router,
  core::{AppConfig, AppRuntime, AppState},
};

#[tokio::main]
async fn main() {
  let app_runtime = AppRuntime::info();

  println!("app_runtime:{app_runtime:?}");

  let config: AppConfig = app_runtime.conf;

  println!("config port:{:?} ", config.port);
  println!("config host:{:?}", config.host);

  let state = AppState::new(config);

  let cors = CorsLayer::new()
    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    .allow_credentials(true)
    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
  let app = create_router().layer(cors).with_state(state);

  println!("🚀 Server started successfully");
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
