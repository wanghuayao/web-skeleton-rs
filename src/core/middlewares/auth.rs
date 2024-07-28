use axum::{
  extract::Request,
  middleware::Next,
  response::{IntoResponse, Response},
};

pub async fn auth_middleware(req: Request, next: Next) -> Result<impl IntoResponse, Response> {
  let authorization = req.headers().get("Authorization");
  if let Some(authorization) = authorization {
    tracing::debug!("Authorization {:?}", authorization.to_str());
  }

  Ok(next.run(req).await)
}
