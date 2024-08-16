use axum::Json;

use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
  status: String,
}
/// health check
pub async fn health_check() -> Json<HealthCheckResponse> {
  Json(HealthCheckResponse {
    status: "OK".to_string(),
  })
}
