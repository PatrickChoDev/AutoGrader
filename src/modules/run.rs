use super::lang;
pub fn run_test(test_file: Option<&str>, input_file: Option<&str>, config: &toml::value::Value) -> () {
    // let parsed = parser::check_test(test_file);
    println!("Running on >> {{}}");
    lang::cpp::compile();
}
