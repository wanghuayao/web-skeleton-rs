use std::{env::current_exe, os::unix::ffi::OsStrExt, path::Path};

use clap::Parser;

use super::{app_args::AppArgs, AppConfig};

#[derive(Debug)]
pub struct AppRuntime {
  /// Name of the App
  pub name: String,

  pub args: AppArgs,

  pub conf: AppConfig,
}

impl AppRuntime {
  pub fn info() -> Self {
    // get exe name
    let exe_path = current_exe().expect("Failed to get current executable path");
    let name = Path::new(&exe_path).file_name().unwrap().as_bytes();
    let name = String::from_utf8(name.to_vec()).unwrap().into();

    let args = AppArgs::parse();

    let config_file = if let Some(file) = args.config.clone() {
      file
    } else {
      format!("{name}.yaml")
    };

    AppRuntime {
      name,
      args: args,
      conf: AppConfig::load(&config_file),
    }
  }
}
