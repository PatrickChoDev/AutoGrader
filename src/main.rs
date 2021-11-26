#![allow(non_snake_case)]

#[macro_use]
extern crate clap;
use clap::App;

mod modules;

fn load_config() -> toml::Value {
    match modules::parser::parse_toml("config.toml") {
        Some(n) => n,
        None => match modules::parser::parse_toml("~/config.toml") {
            Some(n) => n,
            None => {
                panic!("Config file not found!! Please not run AutoGrader as ROOT.");
            }
        },
    }
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let args = App::from_yaml(yaml).get_matches();
    let config = match args.value_of("config") {
        Some(f) => match modules::parser::parse_toml(f) {
            Some(n) => n,
            None => load_config(),
        },
        None => load_config(),
    };
    if let Some(testing) = args.subcommand_matches("test") {}
    if let Some(run) = args.subcommand_matches("run") {
        modules::run::run_test(run.value_of("TEST"), run.value_of("INPUT"),&config);
    }
    if let Some(serve) = args.subcommand_matches("serve") {
        
    }
}
