#![allow(non_snake_case)]

#[macro_use]
extern crate clap;
use clap::App;

mod modules;


fn main() {
    let yaml = load_yaml!("cli.yaml");
    let args = App::from_yaml(yaml).get_matches();
    if let Some(testing) = args.subcommand_matches("test") {}
    if let Some(run) = args.subcommand_matches("run") {
//        modules::run::run_test(run.value_of("TEST"), run.value_of("INPUT"),&config);
    }
    if let Some(serve) = args.subcommand_matches("serve") {
        
    }
}
