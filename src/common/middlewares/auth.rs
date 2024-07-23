use axum::{
  extract::Request,
  middleware::Next,
  response::{IntoResponse, Response},
};

pub async fn auth_middleware(req: Request, next: Next) -> Result<impl IntoResponse, Response> {
  println!("auth_middleware");

  let authorization = req.headers().get("Authorization");
  if let Some(authorization) = authorization {
    println!("a{:?}", authorization.to_str());
  }

  Ok(next.run(req).await)
}
