#![allow(non_snake_case)]

#[macro_use]
extern crate clap;
use clap::App;

mod modules;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let args = App::from_yaml(yaml).get_matches();
    if args.subcommand.is_none() {}
    if let Some(_testing) = args.subcommand_matches("test") {}
    if let Some(_run) = args.subcommand_matches("run") {}
    if let Some(_serve) = args.subcommand_matches("serve") {}
}
