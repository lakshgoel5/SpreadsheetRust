//init_frontend(r, c) -> init_backend(r, c), Print_grid(), run_counter(): returns void
//print grid() -> get_value(value::cell) : returns void
//run_counter -> while loop for argument, process_command(r,c, string), Print_grid() : return void
//display_status
use crate::backend::backend::*;
use crate::common::Value;
use std::time::{Duration, Instant};

pub fn init_frontend(rows: usize, columns: usize) {
    let backend = Backend::init_backend(rows, columns);

    let location = Value::Cell(0, 0); // Starting at (0, 0)
    let dimension = Value::Cell(rows, columns); // Grid size
    print_grid(location, dimension, backend.get_grid());

    run_counter();
}

fn column_decoder(mut j: usize) -> String {
    let mut cc = Vec::new();
    while j > 0 {
        j -= 1;
        cc.push((b'A' + (j % 26) as u8) as char);
        j /= 26;
    }
    cc.reverse();
    cc.into_iter().collect()
}

pub fn print_grid(location: Value, dimension: Value, grid: &Grid) {
    if let (Value::Cell(start_x, start_y), Value::Cell(rows, cols)) = (location, dimension) {
        for i in start_x..start_x + rows {
            for j in start_y..start_y + cols {
                if i == start_x - 1 && j == start_y - 1 {
                    print!("{:>12}", " ");
                } else if i == start_x - 1 {
                    print!("{:>12}", column_decoder(j));
                } else if j == start_y - 1 {
                    print!("{:>12}", i);
                } else {
                    match grid.get_node_value(i, j) {
                        Some(value) => print!("{:>12}", value),
                        None => print!("{:>12}", "ERR"),
                    }
                }
            }
            println!();
        }
    } else {
        eprintln!("Invalid location or dimension values provided.");
    }
}

pub fn display_status(status: Status, elapsed_time: Duration) {
    match status {
        Status::Success => println!("[{:?}] (Ok).", elapsed_time),
        Status::InvalidRange => println!("[{:?}] (Invalid Range).", elapsed_time),
        Status::UnrecognizedCmd => println!("[{:?}] (Unrecognized command)", elapsed_time),
        Status::InvalidRowColumn => println!("[{:?}] (Invalid row or column)", elapsed_time),
        Status::CircularDependency => println!("[{:?}] (Cycle not allowed)", elapsed_time),
        Status::PrintEnabled => println!("[{:?}] (Ok)", elapsed_time),
        Status::PrintDisabled => println!("[{:?}] (Ok)", elapsed_time),
        Status::ScrollTo(row, col) => println!("[{:?}] (Ok)", elapsed_time),
        Status::Up => println!("[{:?}] (Ok)", elapsed_time),
        Status::Down => println!("[{:?}] (Ok)", elapsed_time),
        Status::Left => println!("[{:?}] (Ok)", elapsed_time),
        Status::Right => println!("[{:?}] (Ok)", elapsed_time),
        _ => (),
    }
}

pub fn run_counter() {
    let mut input = String::new();
    let stdin = std::io::stdin();
    let mut backend = Backend::init_backend(10, 10); // Example grid size, adjust as needed

    loop {
        input.clear();
        println!("Enter a command (or 'q' to quit):");
        let start_time = Instant::now();
        stdin.read_line(&mut input).expect("Failed to read input");
        let command = input.trim().to_string();

        if command == "q" {
            let elapsed_time = start_time.elapsed();
            display_status(Status::Quit, elapsed_time);
            break;
        }

        let status = backend.process_command(10, 10, command);
        let elapsed_time = start_time.elapsed();
        display_status(status, elapsed_time);
    }
}

// pub fn display_status(x: i32, time_taken: f64) {
//     print!("[{:.2}] ", time_taken);
//     match x {
//         1 => print!("(ok) > "),  // relevant
//         2 => print!("(invalid range) > "),  // not relevant to autograder - will have to change parser if want to // debug
//         3 => print!("(unrecognized cmd) > "),  // relevant
//         4 => print!("(invalid row/column) > "), // ig not relevant
//         5 => print!("(cycle not allowed) > "), // relevant
//         _ => (),
//     }
//     io::stdout().flush().unwrap();
// }

// fn main() {
//     let mut start_x = 1;
//     let mut start_y = 1;
//     let mut is_disabled = false;
//     let args: Vec<String> = env::args().collect();
//     // part of first processing
//     if !spreadsheet::process_first(args.len(), &args, &mut is_disabled) {
//         return;
//     }
//     let r = args[1].parse::<usize>().unwrap();
//     let c = args[2].parse::<usize>().unwrap();
//     let start = Instant::now();
//     let mut grid = backend::generate_grid(r, c);
//     if !is_disabled {
//         spreadsheet::print_grid(start_x, start_y, r, c, &mut grid);
//     }
//     let duration = start.elapsed();
//     spreadsheet::display_status(1, duration.as_secs_f64());

//     let stdin = io::stdin();
//     let mut command = String::new();

//     // let graph = create_graph(r + 1, c + 1);

//     loop {
//         command.clear();
//         let bytes_read = stdin.lock().read_line(&mut command).unwrap();
//         if bytes_read == 0 {
//             break;
//         }
//         // remove the trailing newline character
//         if command ends_with('\n') {
//             command.pop();
//         }
//         if command is_empty() {
//             spreadsheet::print_grid(start_x, start_y, r, c, &mut grid);
//             print!("[0.0] (unrecognized cmd) > ");
//             io::stdout().flush().unwrap();
//             continue;
//         }

//         let start = Instant::now();
//         // , &graph // debug
//         let status = spreadsheet::process_command(
//             &command,
//             &mut start_x,
//             &mut start_y,
//             r,
//             c,
//             &mut is_disabled,
//             &mut grid,
//         );
//         let duration = start.elapsed();

//         // quit status
//         if status == 0 {
//             break;
//         }
//         spreadsheet::display_status(status, duration.as_secs_f64());
//     }
// }
