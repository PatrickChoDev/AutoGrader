use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TestConfig {
    id: Option<u32>,
    name: String,
    description: Option<String>,
    score: Option<u32>,
    num_testcase: Option<u8>,
    reference: Option<String>,
    file: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
pub struct FileConfig {
    id: Option<u32>,
    file_name: Option<String>,
}

fn parse_file(filename: &str) -> () {}

pub fn check_test(test_file: Option<&str>) -> bool {
    match test_file {
        None => false,
        Some(_file) => true,
    }
}

#[cfg(test)]
#[test]
fn read_file() {
    println!("Test passed");
}

#[test]
fn read_test_file() {
    let config: TestConfig = toml::from_str(
        r#"
    id = 0
    name = 'Test 1'
    score = 100
    num_testcase = 5
    reference = 'filename.pdf'
    file = ['readme.md','hey.txt','lol.py']
    "#,
    )
    .unwrap();
    println!("{:#?}", config);
    println!("Passed!!");
}
