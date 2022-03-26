#![allow(non_snake_case)]

#[macro_use]
extern crate clap;
mod core;
use ansi_term::Colour;
use clap::{Arg, ColorChoice, Command};

fn main() {
  let command = Command::new(crate_name!())
    .color(ColorChoice::Always)
    .subcommand_required(true)
    .help_expected(true)
    .subcommands([
      Command::new("test")
        .about("Test configuration file for AutoGrader")
        .arg(
          Arg::new("input")
            .required(true)
            .takes_value(true)
            .multiple_values(true)
            .help("Config file to check")
            .value_name("file"),
        ),
      Command::new("run")
        .about("Run single test case and solution")
        .args(&[
          Arg::new("test")
            .short('t')
            .takes_value(true)
            .value_name("file")
            .required(true)
            .help("Autograder Test Config"),
          Arg::new("input")
            .short('i')
            .takes_value(true)
            .value_name("file")
            .required(true)
            .help("File to be tested"),
        ]),
    ])
    .get_matches();

  if let Some(testing) = command.subcommand_matches("test") {
    let filenames: Vec<_> = testing.values_of("input").unwrap().collect();
    for filename in filenames {
      match core::parser::parse_ext(filename)
      .ok()
      .unwrap_or_else(|| [""].to_vec())[..]
    {
      ["test.", "json"] => {
        println!(
          "{}  Parse {} as {} config....",
          Colour::Blue.bold().paint("\u{24D8}"),
          Colour::Cyan.dimmed().paint(filename),
          Colour::Purple.bold().paint("Test")
        );
        if core::config::parse_test_config(filename).is_some() {
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
          "{}  Parse {} as {} config....",
          Colour::Blue.bold().paint("\u{24D8}"),
          Colour::Cyan.dimmed().paint(filename),
          Colour::Purple.bold().paint("Session")
        );
        if core::config::parse_root_config(filename).is_some() {
          println!(
            "{}  This file is valid {} config",
            Colour::Green.bold().paint("\u{2714}"),
            Colour::Purple.bold().paint("Session")
          );
        } else {
          println!("{}  Not valid!!!", Colour::Red.bold().paint("\u{2718}"));
        }
      }
      _ => println!("This file isn't AutoGrader Config file"),
    }
    }
  }
  #[allow(unused_must_use)]
  if let Some(run) = command.subcommand_matches("run") {
    async {
      println!(
        "{}",
        core::run::run_test(
          core::config::parse_test_config(run.value_of("test").unwrap()).unwrap(),
          run.value_of("input").unwrap()
        )
        .await
      );
    };
  }
}
