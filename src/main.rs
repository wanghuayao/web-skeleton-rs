use std::str::FromStr;

use axum::{
  extract::{MatchedPath, Request},
  http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
  },
};

use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use web_skeleton::{
  app,
  core::{AppConfig, AppRuntime, AppState},
};

#[tokio::main]
async fn main() {
  let app_runtime = AppRuntime::info();

  let config: AppConfig = app_runtime.conf;
  let state = AppState::new(config);

  // 改进环境变量读取失败的处理
  let filter = match tracing_subscriber::EnvFilter::try_from_default_env() {
    Ok(f) => f,
    Err(_) => {
      tracing_subscriber::EnvFilter::from_str("web_skeleton=debug,tower_http=debug").unwrap()
    }
  };

  tracing_subscriber::registry()
    .with(filter)
    .with(tracing_subscriber::fmt::layer())
    .init();

  let cors = CorsLayer::new()
    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    .allow_credentials(true)
    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

  let trace = TraceLayer::new_for_http()
    .make_span_with(|req: &Request| {
      let method = req.method();
      let path = if let Some(path) = req.extensions().get::<MatchedPath>() {
        path.as_str()
      } else {
        // 确保请求路径的准确性
        req
          .uri()
          .path_and_query()
          .map_or_else(|| "", |pq| pq.path())
      };

      tracing::debug_span!("req",  %method,  %path )
    })
    .on_failure(());

  app::init().await;

  let app = app::create_router()
    .layer(trace)
    .layer(cors)
    .with_state(state);

  let listener = match tokio::net::TcpListener::bind("0.0.0.0:8080").await {
    Ok(l) => l,
    Err(e) => {
      tracing::error!("Failed to bind TCP listener: {}", e);
      std::process::exit(1);
    }
  };

  let addr = listener.local_addr().unwrap();
  tracing::info!("🚀 Server started successfully, listen: on {}", addr);

  axum::serve(listener, app).await.unwrap();
}
