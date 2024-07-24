use config::Config;

#[derive(Debug, Default, serde_derive::Deserialize, PartialEq, Eq)]
pub struct AppConfig {
  pub host: String,
  pub port: String,
}

impl AppConfig {
  pub fn load(config_file: &str) -> Self {
    Config::builder()
      .add_source(config::File::with_name(config_file))
      .add_source(config::Environment::with_prefix("APP"))
      .build()
      .unwrap()
      .try_deserialize()
      .unwrap()
  }
}
