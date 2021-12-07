use serde::{Deserialize, Serialize};
use serde_json::{from_str, Number};

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseConfig {
    host: String,
    port: Number,
    mode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunnerConfig {
    host: String,
    port: Number,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortConfig {
    frontend: Number,
    api: Number,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoGraderConfig {
    name: String,
    database: DatabaseConfig,
    port: PortConfig,
    runner: RunnerConfig,
    test_dir: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestDescriptionConfig {
    reader: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestCasesConfig {
    dir: Option<String>,
    grouping: Option<bool>,
    solution: String,
    pass_all: Option<bool>,
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
    cases: TestCasesConfig,
    limit: Option<TestLimitConfig>,
}

pub fn parse_root_config(filename: &str) -> AutoGraderConfig {
    let buffer = std::fs::read_to_string(filename);
    match buffer {
        Ok(d) => match from_str::<AutoGraderConfig>(&d) {
            Ok(d) => d,
            Err(e) => panic!("Config file cannot parse. Error: {}", e),
        },
        Err(e) => panic!("Config file not found. Error: {}", e),
    }
}

pub fn parse_test_config(filename: &str) -> Option<TestConfig> {
    let buffer = std::fs::read_to_string(filename);
    match buffer {
        Ok(d) => match from_str::<TestConfig>(&d) {
            Ok(d) => Some(d),
            Err(e) => {
                println!("{}", e);
                None
            }
        },
        Err(_) => None,
    }
}

#[cfg(test)]
#[test]
fn load_config() {
    parse_root_config("tests/config.json");
}

#[test]
#[should_panic]
fn load_invalid_config() {
    parse_root_config("wtfconfig.json");
}

#[test]
fn load_test_config() {
    let parsed = parse_test_config("tests/sum.test.json");
    match parsed.as_ref() {
        Some(n) => println!("{:#?}",n),
        None => panic!("")
    }
}
