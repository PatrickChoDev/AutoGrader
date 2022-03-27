#![allow(non_snake_case)]
use std::{env, fs, path::Path};
mod core;

fn main() {
  let args: Vec<String> = env::args().collect();
  let _config_path = match fs::canonicalize(match args.get(1) {
    Some(filepath) => Path::new(filepath),
    None => Path::new("./grader.config.json"),
  }) {
    Ok(n) => n,
    Err(_) => core::exit::AutoGraderExit::ConfigFileNotFound.exit(),
  };
  core::exit::AutoGraderExit::NoError.exit()
}
