use axum::{response::IntoResponse, Json};

use crate::modules::auth::auth_3_services::login_service;

use super::auth_1_models::User;

pub async fn login(Json(user): Json<User>) -> impl IntoResponse {
  println!("handler user: {:?}", user);

  let _ = login_service(&user);

  let json_response = serde_json::json!({
      "status": "success".to_string(),
      "results": 1,
      "auth": vec!["login"],
  });

  Json(json_response)
}

pub async fn register() -> impl IntoResponse {
  let json_response = serde_json::json!({
      "status": "success".to_string(),
      "results": 1,
      "auth": vec!["register"],
  });

  Json(json_response)
}
