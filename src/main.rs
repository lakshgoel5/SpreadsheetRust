// use project::terminal::backend::*;
// use project::terminal::spreadsheet::*;
use std::env;
pub mod terminal;
use crate::terminal::backend::*;
use crate::terminal::spreadsheet::*;
use std::io::{self, BufRead, Write};
use std::time::Instant;

fn main() {
    let mut start_x = 1;
    let mut start_y = 1;
    let mut is_disabled = false;
    let args: Vec<String> = env::args().collect();
    // part of first processing
    if !process_first(args.len(), &args, &mut is_disabled) {
        return;
    }
    let r = args[1].parse::<usize>().unwrap();
    let c = args[2].parse::<usize>().unwrap();
    let start = Instant::now();
    // init frontend
    let mut grid = generate_grid(r, c);
    if !is_disabled {
        print_grid(start_x, start_y, r, c, &mut grid);
    }
    let duration = start.elapsed();
    display_status(1, duration.as_secs_f64());

    let stdin = io::stdin();
    let mut command = String::new();

    // let graph = create_graph(r + 1, c + 1);

    loop {
        command.clear();
        let bytes_read = stdin.lock().read_line(&mut command).unwrap();
        if bytes_read == 0 {
            break;
        }
        // remove the trailing newline character
        if command.ends_with('\n') {
            command.pop();
        }
        if command.is_empty() {
            print_grid(start_x, start_y, r, c, &mut grid);
            print!("[0.0] (unrecognized cmd) > ");
            io::stdout().flush().unwrap();
            continue;
        }

        let start = Instant::now();
        // , &graph // debug
        let status = process_command(
            &command,
            &mut start_x,
            &mut start_y,
            r,
            c,
            &mut is_disabled,
            &mut grid,
        );
        let duration = start.elapsed();

        // quit status
        if status == 0 {
            break;
        }
        display_status(status, duration.as_secs_f64());
    }
}
