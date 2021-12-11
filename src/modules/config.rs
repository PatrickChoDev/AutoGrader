use serde::{Deserialize, Serialize};
use serde_json::{from_str, Map, Number, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseConfig {
    host: String,
    port: Number,
    username: String,
    password: String,
    mode: String,
    scheme: DatabaseScheme,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseScheme {
    db: String,
    user: String,
    score: String,
    group_score: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunnerConfig {
    host: String,
    port: Number,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoGraderConfig {
    name: String,
    database: DatabaseConfig,
    port: Number,
    runner: RunnerConfig,
    test_dir: Option<String>,
    file_dir: Option<String>,
    group: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestDescriptionConfig {
    reader: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestCasesConfig {
    pub dir: Option<String>,
    pub solution: String,
    pass_all: Option<bool>,
    score_weight: Option<Map<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestLimitConfig {
    cpu: Number,
    memory: String,
    time: Number,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestConfig {
    name: String,
    info: Option<String>,
    description: Option<TestDescriptionConfig>,
    pub score: Option<i64>,
    pub cases: TestCasesConfig,
    limit: Option<TestLimitConfig>,
}

pub fn parse_root_config(filename: &str) -> Option<AutoGraderConfig> {
    let buffer = std::fs::read_to_string(filename);
    match buffer {
        Ok(d) => match from_str::<AutoGraderConfig>(&d) {
            Ok(d) => Some(d),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

pub fn parse_test_config(filename: &str) -> Option<TestConfig> {
    let buffer = std::fs::read_to_string(filename);
    match buffer {
        Ok(d) => match from_str::<TestConfig>(&d) {
            Ok(d) => Some(d),
            Err(e) => {
                println!(
                    "{} will be ignored. Because parsing error.\nError: {}",
                    filename, e
                );
                None
            }
        },
        Err(_) => None,
    }
}

#[cfg(test)]
#[test]
fn load_config() {
    assert!(parse_root_config("tests/config.json").is_some());
    assert!(parse_root_config("wtfconfig.json").is_none());
    assert!(parse_test_config("tests/sum.test.json").is_some());
}
