use super::{config, lang, parser};
use ansi_term::Colour;
use async_process::Command;
use std::{fmt, fs::File, path::Path, process::Stdio};

#[derive(PartialEq, Debug)]
pub struct TestScore {
    score: i64,
    max_score: i64,
    marker: Vec<String>,
}

impl fmt::Display for TestScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, m) in self.marker.iter().enumerate() {
            match m.as_str() {
                "P" => write!(
                    f,
                    "Case {}: {}\n",
                    i + 1,
                    Colour::Green.bold().paint("Correct!")
                ),
                "-" => write!(
                    f,
                    "Case {}: {}\n",
                    i + 1,
                    Colour::Red.bold().blink().paint("Wrong Answer")
                ),
                "R" => write!(
                    f,
                    "Case {}: {}\n",
                    i + 1,
                    Colour::Blue.bold().blink().paint("Runtime Error")
                ),
                "?" => write!(
                    f,
                    "Case {}: {}\n",
                    i + 1,
                    Colour::Purple.dimmed().paint("Solution Error")
                ),
                "C" => write!(
                    f,
                    "Case {}: {}\n",
                    i + 1,
                    Colour::White.bold().italic().paint("AutoGrader not support this file extension or language")
                ),
                _ => write!(
                    f,
                    "Case {}: {}\n",
                    i + 1,
                    Colour::Yellow.dimmed().paint("Unknown?")
                ),
            }
            .ok();
        }
        write!(f, "Score: {}/{}", self.score, self.max_score)
    }
}

pub async fn run_test(test_config: config::TestConfig, input_file: &str) -> TestScore {
    let mut score: i64 = 0;
    let mut max_score: i64 = 100;
    let mut marker: Vec<String> = vec![];
    if !Path::new(&input_file).exists() {
        return TestScore {score,max_score,marker:vec!["E".to_string()]}
    }
    let cases = parser::find_testcases(&test_config.cases.dir.unwrap_or_else(|| "".to_string()))
        .unwrap_or_default();
    if cases.is_empty() {
        marker.push(String::from("N"));
    } else {
        match lang::get_exec(input_file).await {
            Some(op) => {
                for case in &cases {
                    let solution_out = match lang::get_exec(&test_config.cases.solution).await {
                        Some(s_op) => match Command::new(&s_op[0])
                            .args(&s_op[1..])
                            .stdin(File::open(Path::new(&case)).unwrap())
                            .stdout(Stdio::piped())
                            .spawn()
                            .ok()
                        {
                            Some(process) => process.output().await.unwrap(),
                            None => {
                                marker.push(String::from("?"));
                                max_score -= &test_config.score.unwrap_or(100) / cases.len() as i64;
                                continue;
                            }
                        },
                        None => {
                            marker.push(String::from("?"));
                            max_score -= &test_config.score.unwrap_or(100) / cases.len() as i64;
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
                    if solution_out.stdout == test_out.stdout {
                        marker.push(String::from("P"));
                        score += &test_config.score.unwrap_or(100) / cases.len() as i64;
                    } else {
                        marker.push(String::from("-"));
                    }
                }
            }
            None => marker.push(String::from("C")),
        };
    }
    if score > max_score {
        score = max_score;
    }
    TestScore {
        score,
        marker,
        max_score,
    }
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
            score: 0,
            max_score: 100,
            marker: vec!["-".to_string(), "-".to_string()]
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
            max_score: 100,
            marker: vec!["P".to_string(), "P".to_string()]
        }
    );
}
