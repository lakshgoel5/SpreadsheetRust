use std::env;
mod backend;
mod common;
mod frontend;
mod parser;
use spreadsheet_rust::frontend::terminal::Frontend;
use std::process::{Command, Stdio};
use std::thread;

/// Entry point of the spreadsheet application.
///
/// By default starts terminal spreadsheet. If the user enters 'web' command,
/// it will launch the web interface in a separate thread while keeping
/// the terminal interface running.
fn main() {
    let args: Vec<String> = env::args().collect();

    // Default rows and columns if not specified
    let rows = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(10)
    } else {
        10
    };

    let columns = if args.len() > 2 {
        args[2].parse::<usize>().unwrap_or(10)
    } else {
        10
    };
    if rows > 999 || rows < 1 {
        return;
    }

    if columns > 18278 || columns < 1 {
        return;
    }

    let mut frontend = Frontend::init_frontend(rows, columns);
    frontend.run_frontend();
}
