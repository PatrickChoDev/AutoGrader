use super::{config, parser};
use nanoid::nanoid;
use std::process::{Command, Stdio};

trait LanguageOperation {
    fn save_correct(
        test_file: &str,
        mut case_config: config::TestCasesConfig,
    ) -> config::TestCasesConfig {
        case_config
    }
    fn run(input_file: &str, case_config: config::TestCasesConfig) -> Result<u32, String> {
        Ok(0)
    }
}

pub fn get_exec(filename: &str) ->  Option<Vec<String>> {
    let mut tmp_path = String::from("/tmp/solution-");
    tmp_path.push_str(nanoid!(32).as_str());
    match parser::parse_ext(&filename).unwrap().last().unwrap() {
        &"rs" => {
            Command::new("rustc")
                .args([&filename, "-o", &tmp_path])
                .current_dir("/")
                .spawn()
                .ok()
                .expect("Cannot compile").try_wait().unwrap();                
            Command::new("chmod").args(["777",&tmp_path]).spawn().ok().expect("Cannot set file permission");
            Some([tmp_path].to_vec())
        }
        &"py" | &"py3" => Some(["python".to_string(), filename.to_string()].to_vec()),
        &"c" => {
            Command::new("gcc")
                .args([&filename, "-o", &tmp_path])
                .current_dir("/")
                .spawn()
                .ok()
                .expect("Cannot compile");
            Command::new("chmod").args(["777",&tmp_path]).spawn().ok().expect("Cannot set file permission");
            Some([tmp_path].to_vec())
        }
        &"cpp" => {
            Command::new("g++")
                .args([&filename, "-o", &tmp_path])
                .current_dir("/")
                .spawn()
                .ok()
                .expect("Cannot compile").try_wait().unwrap();
            Command::new("chmod").args(["777",&tmp_path]).spawn().ok().expect("Cannot set file permission").try_wait().unwrap();
            Some([tmp_path].to_vec())
        }
        _ => None,
    }
}
