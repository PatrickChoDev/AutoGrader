use super::parser;
use async_process::Command;
use nanoid::nanoid;
use std::path::Path;

pub async fn get_exec(filename: &str) -> Option<Vec<String>> {
    let mut tmp_path = String::from("/tmp/solution-");
    tmp_path.push_str(nanoid!(32).as_str());
    let tmp_path = Path::new(tmp_path.as_str()).to_str().unwrap();
    match parser::parse_ext(filename).unwrap().last().unwrap() {
        &"rs" => {
            if Command::new("rustc")
                .args([filename, "-o", tmp_path])
                .status()
                .await
                .ok()
                .unwrap()
                .success()
            {
                Some([tmp_path.to_string()].to_vec())
            } else {
                None
            }
        }
        &"py" | &"py3" => Some(["python".to_string(), filename.to_string()].to_vec()),
        &"c" => {
            if Command::new("gcc")
                .args([filename, "-o", tmp_path])
                .status()
                .await
                .ok()
                .unwrap()
                .success()
            {
                Some([tmp_path.to_string()].to_vec())
            } else {
                None
            }
        }
        &"cpp" => {
            if Command::new("g++")
                .args([filename, "-o", tmp_path])
                .status()
                .await
                .ok()
                .unwrap()
                .success()
            {
                Some([tmp_path.to_string()].to_vec())
            } else {
                None
            }
        }
        _ => None,
    }
}
