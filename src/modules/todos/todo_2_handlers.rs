use axum::{response::IntoResponse, Json};

pub async fn list() -> impl IntoResponse {
  let json_response = serde_json::json!({
      "status": "success".to_string(),
      "results": 1,
      "todos": vec!["a"],
  });

  Json(json_response)
}
