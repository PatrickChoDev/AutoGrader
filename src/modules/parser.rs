use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Deserialize,Serialize,Debug)]
pub struct DescriptionType {
    filetype: Option<String>,
    path: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TestConfig {
    id: Option<u32>,
    name: String,
    info: Option<String>,
    description: Option<DescriptionType>,
    score: Option<u32>,
    num_testcase: Option<u8>,
    files: Option<Vec<String>>,
}

pub trait GraderConfig {
    fn into() {
        println!("Hello")
    }
}

fn parse_toml(filename: &str) -> Option<Value> {
    let filebuf = std::fs::read_to_string(filename);
    if filebuf.as_ref().is_err() {
        println!("{} : {}",filename,filebuf.as_ref().unwrap_err());
        None
    }
    else {
        Some(filebuf.ok().unwrap().parse::<Value>().unwrap())
    }
}

pub fn check_testconfig(test_file: Option<&str>) -> bool {
    match test_file {
        None => false,
        Some(_) => true,
    }
}

#[cfg(test)]
#[test]
fn read_file() {
    let fileOkay = parse_toml("Cargo.toml");
    assert!(fileOkay.is_some());
    let fileNotOkay = parse_toml("NoFileFound.toml");
    assert!(fileNotOkay.is_none());
}

#[test]
fn read_test_file() {
}
