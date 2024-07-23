#[derive(Debug, Default, serde_derive::Deserialize, PartialEq, Eq)]
pub struct AppConfig {
  pub host: String,
  pub port: String,
}
