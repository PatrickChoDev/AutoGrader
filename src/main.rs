#![allow(non_snake_case)]
use std::{env, fs, path::Path};

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
  let path = match fs::canonicalize(match args.get(1) {
    Some(filepath) => Path::new(filepath),
    None => Path::new("./grader.config.json"),
  }) {
    Ok(filename) => filename,
    Err(_) => panic!("File not found"),
  };
  println!("{:?}", path);
}
