#![allow(non_snake_case)]
use std::{env, fs, path::Path};
mod core;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config_path = match fs::canonicalize(match args.get(1) {
    Some(filepath) => Path::new(filepath),
    None => Path::new("./grader.config.json"),
  }) {
    Ok(n) => n.into_boxed_path(),
    Err(_) => core::exit::AutoGraderExit::ConfigFileNotFound.exit(),
  };
  let _file = core::files::AutoGraderConfig::load_from(config_path);
  core::exit::AutoGraderExit::NoError.exit()
}
