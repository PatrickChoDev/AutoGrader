use super::{config, parser};
use uuid::Uuid;

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

pub fn get_operation(filename: &str) -> Option<String> {
    let mut op = String::new();
    match parser::parse_ext(&filename).unwrap().last().unwrap() {
        &"rs" => {
            std::fmt::write(
                &mut op,
                format_args!("rustc {} -o /tmp/solution-{}", &filename, Uuid::new_v4()),
            )
            .expect("Error occurred while trying to get test case operation");
        }
        &"py" | &"py3" => {
            std::fmt::write(&mut op, format_args!("python {}", &filename))
                .expect("Error occurred while trying to get test case operation");
        }
        &"c" => {
            std::fmt::write(
                &mut op,
                format_args!("gcc {} -o /tmp/solution-{}", &filename, Uuid::new_v4()),
            )
            .expect("Error occurred while trying to get test case operation");
        }
        &"cpp" => {
            std::fmt::write(
                &mut op,
                format_args!("g++ {} -o /tmp/solution-{}", &filename, Uuid::new_v4()),
            )
            .expect("Error occurred while trying to get test case operation");
        }
        _ => (),
    };
    if op.len() == 0 {
        return None;
    }
    Some(op)
}
