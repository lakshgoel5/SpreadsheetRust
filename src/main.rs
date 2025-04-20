use std::env;
mod backend;
mod common;
mod frontend;
mod parser;
use crate::frontend::web::start_web_app;
use spreadsheet_rust::frontend::terminal::Frontend;
use std::process::{Command, Stdio};
use std::thread;

/// Entry point of the spreadsheet application.
///
/// By default starts terminal spreadsheet. If the user enters 'web' command,
/// it will launch the web interface in a separate thread while keeping
/// the terminal interface running.
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    let args: Vec<String> = env::args().collect();

    // Default rows and columns if not specified
    #[cfg(not(target_arch = "wasm32"))]
    let rows: isize = if args.len() > 1 {
        args[1].parse::<isize>().unwrap_or(10)
    } else {
        10
    };

    #[cfg(not(target_arch = "wasm32"))]
    let columns: isize = if args.len() > 2 {
        args[2].parse::<isize>().unwrap_or(10)
    } else {
        10
    };
    #[cfg(not(target_arch = "wasm32"))]
    if rows > 999 || rows < 1 {
        return;
    }
    #[cfg(not(target_arch = "wasm32"))]
    if columns > 18278 || columns < 1 {
        return;
    }
    #[cfg(not(target_arch = "wasm32"))]
    let mut frontend = Frontend::init_frontend((rows as usize), (columns as usize));
    #[cfg(not(target_arch = "wasm32"))]
    frontend.run_frontend();


    #[cfg(target_arch = "wasm32")]
    start_web_app();
}