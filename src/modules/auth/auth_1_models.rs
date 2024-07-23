use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
  pub username: String,
  pub password: String,
}
