use super::config;
use serde_json::{from_str};

pub fn load_root_config(filename: &str) -> config::root::AutoGraderConfig {
    let data = std::fs::read_to_string(filename);
    match data {
        Ok(d) => match from_str::<config::root::AutoGraderConfig>(&d) {
            Ok(d) => d,
            Err(e) => panic!("{}", e),
        },
        Err(e) => panic!("{}", e),
    }
}

#[cfg(test)]
#[test]
fn load_config() {
    load_root_config("tests/config.json");
}
