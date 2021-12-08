use super::{lang,config};
pub fn run_test(test_config: config::TestConfig, input_file: &str) -> u32 {
    let ops = lang::get_operation(input_file);
    match ops {
        Some(op) => {
            println!("{}",op);
            100
        },
        None => {
            println!("There's no language support for this input");
            0
        }
    }
}


#[cfg(test)]
#[test]
fn run_test_file() {
    use super::config;
    let config = config::parse_test_config("tests/sum.test.json").unwrap();
    run_test(config, "tests/sum/sol.cpp");
}