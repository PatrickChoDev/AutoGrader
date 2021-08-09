#![allow(non_snake_case)]
mod term;
mod structure;

fn main() {
    term::display("Main Menu");
    term::change_menu();
    term::detect_key();
}