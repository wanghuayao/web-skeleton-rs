use clap::{arg, command, Parser};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct AppArgs {
  /// Config file
  #[arg(short, long)]
  pub config: Option<String>,
}
