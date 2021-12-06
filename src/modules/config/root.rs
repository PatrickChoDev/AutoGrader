use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Result};

#[derive(Serialize, Deserialize)]
pub struct DatabaseConfig {
    host: String,
    port: Number,
    mode: String,
}

#[derive(Serialize, Deserialize)]
pub struct RunnerConfig {
    host: String,
    port: Number,
}

#[derive(Serialize, Deserialize)]
pub struct PortConfig {
    frontend: Number,
    api: Number,
}

#[derive(Serialize, Deserialize)]
pub struct AutoGraderConfig {
    name: String,
    database: DatabaseConfig,
    port: PortConfig,
    runner: RunnerConfig,
    test_dir: String,
}
