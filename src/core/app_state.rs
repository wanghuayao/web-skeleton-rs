use std::sync::Arc;

use super::AppConfig;

#[derive(Default, Clone)]
pub struct AppState {
  pub config: Arc<AppConfig>,
}

impl AppState {
  /// 创建一个新的 `AppState` 实例，其中 `config` 使用 `Arc` 进行管理以支持多线程访问。
  pub fn new(config: AppConfig) -> Self {
    AppState {
      config: Arc::new(config), // 使用 Arc 来支持跨线程共享配置
    }
  }
}
