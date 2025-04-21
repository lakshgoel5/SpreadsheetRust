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
    // let args: Vec<String> = env::args().collect();
    //
    // // Default rows and columns if not specified
    // let rows = if args.len() > 1 {
    //     args[1].parse::<usize>().unwrap_or(10)
    // } else {
    //     10
    // };
    //
    // let columns = if args.len() > 2 {
    //     args[2].parse::<usize>().unwrap_or(10)
    // } else {
    //     10
    // };
    // if !(1..=999).contains(&rows) {
    //     return;
    // }
    //
    // if !(1..=18278).contains(&columns) {
    //     return;
    // }
    //
    // let mut frontend = Frontend::init_frontend(rows, columns);
    // frontend.run_frontend();
    #[cfg(not(target_arch = "wasm32"))]
    let args: Vec<String> = env::args().collect();

    // Default rows and columns if not specified
    #[cfg(not(target_arch = "wasm32"))]
    let rows = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(10)
    } else {
        10
    };

    #[cfg(not(target_arch = "wasm32"))]
    let columns = if args.len() > 2 {
        args[2].parse::<usize>().unwrap_or(10)
    } else {
        10
    };
    #[cfg(not(target_arch = "wasm32"))]
    if !(1..=999).contains(&rows) {
        return;
    }
    #[cfg(not(target_arch = "wasm32"))]
    if !(1..=18278).contains(&columns) {
        return;
    }
    #[cfg(not(target_arch = "wasm32"))]
    let mut frontend = Frontend::init_frontend(rows, columns);
    #[cfg(not(target_arch = "wasm32"))]
    frontend.run_frontend();


    #[cfg(target_arch = "wasm32")]
    start_web_app();
}
