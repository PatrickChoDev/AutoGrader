#![allow(non_snake_case)]
use ansi_term::Colour;
use clap::{App, AppSettings, Arg, SubCommand};
mod modules;

#[tokio::main]
async fn main() {
    let args = App::new("AutoGrader")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .version("1.0.1")
        .about("Programming Competition Scoreboard")
        .author("Thanapat Chotipun <devpatrick.cho@gmail.com>")
        .subcommands([
            SubCommand::with_name("test")
                .setting(AppSettings::ColoredHelp)
                .about("Test configuration file for AutoGrader")
                .arg(
                    Arg::with_name("input")
                        .required(true)
                        .takes_value(true)
                        .help("Config file to check")
                        .value_name("file"),
                ),
            SubCommand::with_name("run")
                .setting(AppSettings::ColoredHelp)
                .about("Run single test case and solution")
                .args(&[
                    Arg::with_name("test")
                        .short("t")
                        .takes_value(true)
                        .value_name("file")
                        .required(true)
                        .help("Autograder Test Config"),
                    Arg::with_name("input")
                        .short("i")
                        .takes_value(true)
                        .value_name("file")
                        .required(true)
                        .help("File to be tested"),
                ]),
        ])
        .get_matches();
    if let Some(testing) = args.subcommand_matches("test") {
        let filename = testing.value_of("input").unwrap();
        match modules::parser::parse_ext(filename)
            .ok()
            .unwrap_or_else(|| [""].to_vec())[..]
        {
            ["test.", "json"] => {
                println!(
                    "{}  Parse as {} config....",
                    Colour::Blue.bold().paint("\u{24D8}"),
                    Colour::Purple.bold().paint("Test")
                );
                if modules::config::parse_test_config(filename).is_some() {
                    println!(
                        "{}  This file is valid {} config",
                        Colour::Green.bold().paint("\u{2714}"),
                        Colour::Purple.bold().paint("Test")
                    );
                } else {
                    println!("{}  Not valid!!!", Colour::Red.bold().paint("\u{2718}"));
                }
            }
            ["json"] => {
                println!(
                    "{}  Parse as {} config....",
                    Colour::Blue.bold().paint("\u{24D8}"),
                    Colour::Purple.bold().paint("Session")
                );
                if modules::config::parse_root_config(filename).is_some() {
                    println!(
                        "{}  This file is valid {} config",
                        Colour::Green.bold().paint("\u{2714}"),
                        Colour::Purple.bold().paint("Session")
                    );
                } else {
                    println!("{}  Not valid!!!", Colour::Red.bold().paint("\u{2718}"));
                }
            }
            _ => println!("This file isn't AutoGraderConfig file"),
        }
    }
    if let Some(run) = args.subcommand_matches("run") {
        println!(
            "{}",
            modules::run::run_test(
                modules::config::parse_test_config(run.value_of("test").unwrap()).unwrap(),
                run.value_of("input").unwrap()
            )
            .await
        );
    }
}
