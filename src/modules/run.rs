use super::{config, lang, parser};
use async_process::Command;
use std::{fs::File, path::Path, process::Stdio};

#[derive(Debug, PartialEq)]
pub struct TestScore {
    score: u32,
    marker: Vec<String>,
}

pub async fn run_test(test_config: config::TestConfig, input_file: &str) -> TestScore {
    let mut score: u32 = 0;
    let mut marker: Vec<String> = vec![];
    let cases = parser::find_testcases(&test_config.cases.dir.unwrap_or_else(|| "".to_string()))
        .unwrap_or_default();
    if cases.is_empty() {
        marker.push(String::from("N"));
    } else {
        match lang::get_exec(input_file).await {
            Some(op) => {
                for case in &cases {
                    let solution_out = match Command::new(&op[0])
                        .args(&op[1..])
                        .stdin(File::open(Path::new(&case)).unwrap())
                        .stdout(Stdio::piped())
                        .spawn()
                        .ok()
                    {
                        Some(process) => process.output().await.unwrap(),
                        None => {
                            marker.push(String::from("?"));
                            continue;
                        }
                    };
                    let test_out = match Command::new(&op[0])
                        .args(&op[1..])
                        .stdin(File::open(Path::new(&case)).unwrap())
                        .stdout(Stdio::piped())
                        .spawn()
                        .ok()
                    {
                        Some(process) => process.output().await.unwrap(),
                        None => {
                            marker.push(String::from("R"));
                            continue;
                        }
                    };
                    if solution_out == test_out {
                        marker.push(String::from("P"));
                        score += 100 / cases.len() as u32;
                    } else {
                        marker.push(String::from("-"));
                    }
                }
            }
            None => marker.push(String::from("C")),
        };
    }
    if score > 100 {
        score = 100;
    }
    TestScore { score, marker }
}

#[cfg(test)]
#[tokio::test]
async fn run_test_file() {
    use super::config;
    assert_eq!(
        run_test(
            config::parse_test_config("tests/sum.test.json").unwrap(),
            "tests/sum/wsol.py"
        )
        .await,
        TestScore {
            score: 100,
            marker: vec!["P".to_string(), "P".to_string()]
        }
    );
    assert_eq!(
        run_test(
            config::parse_test_config("tests/sum.test.json").unwrap(),
            "tests/sum/sum.cpp"
        )
        .await,
        TestScore {
            score: 100,
            marker: vec!["P".to_string(), "P".to_string()]
        }
    );
}
