use termion::{clear, color, style, cursor};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write,stdout};
fn flush_screen() -> () {
    println!("{}", clear::All)
}

fn print_header(page: &str) -> () {
    println!(
        "{}{}AUTOGRADER!! {}>>> {}{}{}\n=================================",
        cursor::Goto(1,1),
        style::Bold,
        color::Fg(color::Green),
        color::Fg(color::Yellow),
        page,
        style::Reset
    );
}

pub fn display(page: &str) -> () {
    flush_screen();
    print_header(page);
}

pub fn detect_key() -> String {
    let terminal = stdout().into_raw_mode();
    let stdout = terminal.unwrap();

    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();
    let mut s = String::new();

    loop {
        // Read input (if any)
        let input = stdin.next();

        // If a key was pressed
        if let Some(Ok(key)) = input {
            match key {
                // Exit if 'q' is pressed
                termion::event::Key::Char('q') => break,
                // Else print the pressed key
                _ => {
                    if let termion::event::Key::Char(k) = key {
                        s.push(k);
                  }
                    stdout.lock().flush().unwrap();
                }
            }
        }
    }
    s
}

fn recieve_line(s: &str) -> String {
    print!("{}{} : {}",style::Bold, s, style::Reset);
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    match std::io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}
pub fn change_menu() -> () {
    println!("[R] Restore from session file");
    println!("[I] Add {}Test cases / Input{}",color::Fg(color::Green),style::Reset);
    println!("[O] Add {}ground truth output{} file",color::Fg(color::Yellow),style::Reset);
    println!("[T] {}Test a file{}",color::Fg(color::Blue),style::Reset);
    println!("{}[Q] for Exit.{}",color::Fg(color::Red),style::Reset);
    println!("{}Press a key to use..{}",color::Fg(color::Rgb(150,150,150)),style::Reset)
}


