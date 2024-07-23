use axum::http::{
  header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
  HeaderValue, Method,
};
use config::Config;
use tower_http::cors::CorsLayer;
use web_skeleton::{app::create_router, common::AppConfig};

#[tokio::main]
async fn main() {
  let config = Config::builder()
    .add_source(config::File::with_name("web-skeleton-rs.yaml"))
    .add_source(config::Environment::with_prefix("APP"))
    .build()
    .unwrap();

  let config: AppConfig = config.try_deserialize().unwrap();

  println!("config port:{:?} ", config.port);
  println!("config host:{:?}", config.host);

  let cors = CorsLayer::new()
    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    .allow_credentials(true)
    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

  let app = create_router().layer(cors);

  println!("🚀 Server started successfully");
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
