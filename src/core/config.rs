use super::parser;
use std::path::Path;

#[derive(Debug,serde::Deserialize)]
pub struct AutoGraderConfig {
  pub id: i32,
}

pub trait ConfigFiles {
  fn load_from(path: Box<Path>) -> Option<Box<Self>>;
}

impl AutoGraderConfig {
  pub fn load_from(path: Box<Path>) -> Option<AutoGraderConfig> {
    match std::fs::read_to_string(path) {
      Ok(n) => match parser::parse_string::<AutoGraderConfig>(n) {
        Ok(config) => Some(config),
        Err(_) => None
      },
      Err(e) => {
        println!("{}", e);
        None
      }
    }
  }
}
