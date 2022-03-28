use std::path::Path;

#[derive(Debug)]
pub struct AutoGraderConfig {
  pub id: i32,
}

pub trait ConfigFiles {
  fn load_from(path: Box<Path>) -> Option<Box<Self>>;
}

impl AutoGraderConfig {
  pub fn load_from(path: Box<Path>) -> Option<Box<AutoGraderConfig>> {
    match std::fs::read_to_string(path) {
      Ok(n) => Some(Box::new(AutoGraderConfig { id: 12 })),
      Err(e) => {
        println!("{}", e);
        None
      }
    }
  }
}
