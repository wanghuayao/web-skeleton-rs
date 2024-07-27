use std::sync::Arc;

use super::AppConfig;

#[derive(Default, Clone)]
pub struct AppState {
  pub config: Arc<AppConfig>,
}

impl AppState {
  pub fn new(config: AppConfig) -> Self {
    AppState {
      config: Arc::new(config),
    }
  }
}
