use serde::{Deserialize, Serialize};
use std::fs;
use toml::Value;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DescriptionType {
    filetype: String,
    path: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GraderConfig {
    broadcast_port: Option<String>,
    name: Option<String>,
    db_host: Option<String>,
    db_type: Option<String>,
    runner_host: Option<String>,
    runner_port: Option<String>,
    test_directory: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TestConfig {
    id: Option<u32>,
    name: String,
    info: Option<String>,
    description: Option<DescriptionType>,
    score: Option<u32>,
    case: Option<Vec<TestCase>>,
    case_sep: String,
    limit: Vec<String>,
    trim_line: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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

enum MemSuffix {
    B(i64),
    K(i64),
    M(i64),
    G(i64),
}

impl MemSuffix {
    fn to_num(&self) -> i64 {
        match self {
            Suffix::B(n) => *n,
            Suffix::K(n) => *n * 1024,
            Suffix::M(n) => *n * (1024 as i64).pow(2),
            Suffix::G(n) => *n * (1024 as i64).pow(3),
        }
    }
    fn from(s: &str) -> Result<Self, &str> {
        if s.ends_with("K") {
            match s.split_at(s.len()).0.parse::<i64>() {
                Ok(n) => Ok(Self::K(n)),
                Err(_) => Err("Suffix can't parse"),
            }
        } else if s.ends_with("M") {
            match s.split_at(s.len()).0.parse::<i64>() {
                Ok(n) => Ok(Self::M(n)),
                Err(_) => Err("Suffix can't parse"),
            }
        } else {
            match s.split_at(s.len()).0.parse::<i64>() {
                Ok(n) => Ok(Self::B(n)),
                Err(_) => Err("Suffix can't parse"),
            }
        }
    }
}

enum TimeSuffix {
    Mi(i64),
    S(i64),
    M(i64),
    H(i64),
}

impl TimeSuffix {
    fn to_num(&self) -> f64 {
        match self {
            TimeSuffix::Mi(n) => *n as f64 / 1000.0,
            TimeSuffix::S(n) => *n as f64,
            TimeSuffix::M(n) => *n as f64 * 60.0,
            TimeSuffix::H(n) => *n as f64 * 3600.0,
        }
    }
    fn from(s: &str) -> Result<Self, &str> {
        if s.ends_with("H") {
            match s.split_at(s.len()).0.parse::<i64>() {
                Ok(n) => Ok(Self::H(n)),
                Err(_) => Err("Suffix can't parse"),
            }
        } else if s.ends_with("S") {
            match s.split_at(s.len()).0.parse::<i64>() {
                Ok(n) => Ok(Self::S(n)),
                Err(_) => Err("Suffix can't parse"),
            }
        } else if s.ends_with("M") {
            match s.split_at(s.len()).0.parse::<i64>() {
                Ok(n) => Ok(Self::M(n)),
                Err(_) => Err("Suffix can't parse"),
            }
        } else {
            match s.split_at(s.len()-1).0.parse::<i64>() {
                Ok(n) => Ok(Self::Mi(n)),
                Err(_) => Err("Suffix can't parse"),
            }
        }
    }
}

pub fn parse_case(
    inp_filename: &str,
    out_filename: &str,
    config: &TestConfig,
) -> Option<TestConfig> {
    let inp_filebuf = std::fs::read_to_string(inp_filename);
    let out_filebuf = std::fs::read_to_string(out_filename);
    if (&inp_filebuf).is_err() || (&out_filebuf).is_err() {
        println!("Cannot Parse Test Cases");
        None
    } else {
        let mut cases: Vec<TestCase> = vec![];
        let test_in = inp_filebuf.ok().unwrap();
        let test_out = out_filebuf.ok().unwrap();
        let mut tmp_stdin: String = "".to_owned();

        for line in test_in.lines() {
            if !line.starts_with(config.case_sep.as_str()) {
                tmp_stdin.push_str(line);
                tmp_stdin.push_str("\n");
            } else {
                let info: Vec<&str> = line.split_inclusive(config.case_sep.as_str()).collect();
                let time_limit = match TimeSuffix::from(info[0]) {
                    Ok(n) => n,
                    Err(_) => match TimeSuffix::from(&config.limit[0].to_owned()) {
                        Ok(n) => n,
                        Err(_) => panic!("Config atrribrute 'limit[0]' (Time Limit) can't parse"),
                    },
                };
                let mem_limit = match MemSuffix::from(info[1]) {
                    Ok(n) => n,
                    Err(_) => match MemSuffix::from(&config.limit[1].to_owned()) {
                        Ok(n) => n,
                        Err(_) => panic!("Config atrribrute 'limit[1]' (Mem Limit) can't parse"),
                    },
                };
                let id = cases.len() as u32;
                cases.push(TestCase(id: id,inp: tmp_stdin.to_owned()))
            }
        }
        Some(config.clone())
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
fn read_case() {
    parse_case(
        "tests/sum/sum.case.test",
        "tests/sum/sum.out.test",
        &parse_toml("tests/sum/sum.test.toml")
            .unwrap()
            .try_into::<TestConfig>()
            .ok()
            .unwrap(),
    );
}

#[test]
fn suffix_convert() {
    let a = Suffix::M(100);
    println!("{}", a.to_num());
}
