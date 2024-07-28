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
  app::create_router,
  core::{AppConfig, AppRuntime, AppState},
};

#[tokio::main]
async fn main() {
  let app_runtime = AppRuntime::info();

  let config: AppConfig = app_runtime.conf;

  let state = AppState::new(config);

  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "web_skeleton=debug,tower_http=debug".into()),
    )
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
      // let uri: &axum::http::Uri = req.uri();

      // let matched_path = req
      //   .extensions()
      //   .get::<MatchedPath>()
      //   .map(|matched_path| matched_path.as_str());

      let path = if let Some(path) = req.extensions().get::<MatchedPath>() {
        path.as_str()
      } else {
        req.uri().path()
      };

      tracing::debug_span!("req",  %method,  %path )
    })
    // By default `TraceLayer` will log 5xx responses but we're doing our specific
    // logging of errors so disable that
    .on_failure(());

  let app = create_router().layer(trace).layer(cors).with_state(state);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

  let addr = listener.local_addr().unwrap();
  tracing::info!("🚀 Server started successfully, listen: on {}", addr);

  axum::serve(listener, app).await.unwrap();
}
