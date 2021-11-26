use serde::{Deserialize, Serialize};
use toml::Value;
use std::fs;

#[derive(Deserialize, Serialize, Debug)]
pub struct DescriptionType {
    filetype: String,
    path: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GraderConfig {
    broadcast_port:Option<String>,
name:Option<String>,
db_host:Option<String>,
db_type: Option<String>,
runner_host:Option<String>,
runner_port:Option<String>,
test_directory: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TestConfig {
    id: Option<u32>,
    name: String,
    info: Option<String>,
    description: Option<DescriptionType>,
    score: Option<u32>,
    case: Option<Vec<TestCase>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TestCase {
    id: Option<u32>,
    inp: Option<String>,
    out: Option<String>,
    score: u32,
    cpu_limit: Option<i32>,
    mem_limit: Option<String>,
    time_limit: Option<i32>,
}

pub fn parse_toml(filename: &str) -> Option<Value> {
    let filebuf = std::fs::read_to_string(filename);
    if filebuf.as_ref().is_err() {
        println!("{} : {}", filename, filebuf.as_ref().unwrap_err());
        None
    } else {
        Some(filebuf.ok().unwrap().parse::<Value>().unwrap())
    }
}

pub fn check_testconfig(test_file: Option<&str>) -> bool {
    match test_file {
        Some(file) => match parse_toml(file) {
            Some(t) => match t.try_into::<TestConfig>() {
                Ok(_) => true,
                Err(_) => false,
            },
            None => false,
        },
        None => false,
    }
}

pub fn parse_case(inp_filename: &str, out_filename: &str,config : &GraderConfig) -> Option<TestCase> {
    let inp_filebuf = std::fs::read_to_string(inp_filename);
    let out_filebuf = std::fs::read_to_string(out_filename);
    if (&inp_filebuf).is_err() || (&out_filebuf).is_err(){
        println!("Cannot Parse Test Cases");
        None
    } else {
        let mut cases: Vec<TestCase> = vec![];
        let test_in = inp_filebuf.ok().unwrap();
        let test_out = out_filebuf.ok().unwrap();
        let mut tmp_stdin = "";
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
    assert_eq!(check_testconfig(Some("tests/sum/sum.test.toml")), true)
}

#[test]
fn list_case() {
    search_cases("tests/")
}
