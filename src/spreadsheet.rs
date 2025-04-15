use crate::backend::generate_grid;
use crate::graph::Node;
use crate::parser;
use crate::functions::Value;
use std::cmp;
use std::env;
use std::ffi::CString;
use std::io::{self, BufRead, Write};
use std::ptr;
use std::time::Instant;
use crate::backend::getting_things_updated;
const MAX_ROW: usize = 999;
const MAX_COLUMN: usize = 18278;

// static mut IS_DISABLED: bool = false;
// debug // change this
// static mut GRID: Option<Vec<Vec<i32>>> = None;
//
// fn generate_grid(r: usize, c: usize) {
//     unsafe {
//         GRID = Some(vec![vec![0; c + 1]; r + 1]);
//     }
// }

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

// , graph: &Graph
fn print_grid(
    start_x: usize,
    start_y: usize,
    r: usize,
    c: usize,
    grid: &mut Vec<Vec<crate::graph::Node>>,
) {
    let max_x = cmp::min(9 + start_x, r);
    let max_y = cmp::min(9 + start_y, c);

    for i in start_x - 1..=max_x {
        for j in start_y - 1..=max_y {
            if i == start_x - 1 && j == start_y - 1 {
                print!("{:>12}", " ");
            } else if i == start_x - 1 && j != start_y - 1 {
                print!("{:>12}", column_decoder(j));
            } else if j == start_y - 1 {
                print!("{:>12}", i);
            } else {
                print!("{:>12}", grid[i][j]);
                // unsafe {
                //     print!("{:>12}", GRID.as_ref().unwrap()[i][j]);
                // }
                // print!("{:>12}", GRID.as_ref().unwrap()[i][j]);
                // if graph.matrix[i][j].is_none() {
                //     print!("{:>12}", 0);
                // }
                // // Option<T> -> Option<&T> ->
                // else if graph.matrix[i][j].as_ref().unwrap().valid {
                //     unsafe {
                //         print!("{:>12}", GRID.as_ref().unwrap()[i][j]);
                //     }
                // }
                // else {
                //     print!("{:>12}", "ERR");
                // }
            }
        }
        println!();
    }
}

fn display_status(x: i32, time_taken: f64) {
    print!("[{:.2}] ", time_taken);
    match x {
        1 => print!("(ok) > "),
        2 => print!("(invalid range) > "),
        3 => print!("(unrecognized cmd) > "),
        4 => print!("(invalid row/column) > "),
        5 => print!("(cycle not allowed) > "),
        6 => print!("(invalid range) > "),
        _ => (),
    }
    io::stdout().flush().unwrap();
}

fn is_number(str: &str) -> bool {
    !str.is_empty() && str.chars().all(|c| c.is_ascii_digit())
}

// , graph: &Graph // debug
// this processes commands and prints the grid
fn process_command(
    command: &str,
    start_x: &mut usize,
    start_y: &mut usize,
    r: usize,
    c: usize,
    is_disabled: &mut bool,
    grid: &mut Vec<Vec<Node>>,
) -> i32 {
    match command {
        "q" => return 0,
        "w" => {
            *start_x = if *start_x > 10 { *start_x - 10 } else { 1 };
            if !(*is_disabled) {
                print_grid(*start_x, *start_y, r, c, grid);
            }
            return 1;
        }
        "s" => {
            if *start_x + 10 <= r {
                *start_x += 10;
                *start_x = cmp::min(*start_x, r - 9);
            }
            if !(*is_disabled) {
                print_grid(*start_x, *start_y, r, c, grid);
            }
            return 1;
        }
        "a" => {
            *start_y = if *start_y > 10 { *start_y - 10 } else { 1 };
            if !(*is_disabled) {
                print_grid(*start_x, *start_y, r, c, grid);
            }
            return 1;
        }
        "d" => {
            if *start_y + 10 <= c {
                *start_y += 10;
                *start_y = cmp::min(*start_y, c - 9);
            }
            if !(*is_disabled) {
                print_grid(*start_x, *start_y, r, c, grid);
            }
            return 1;
        }
        _ => {}
    }

    // let mut function: isize = -1;
    // work with enums only not functions

    match parser::validate(command, &r, &c) {
        Some((Some(Value::Cell(row, col)), Some(Value::Oper(v1, v2, op)))) => {
            let target_cell = Value::Cell(row, col);
            let val1 = v1.as_ref();
            let val2 = v2.as_ref();
            let operation = op;
    
            let status = getting_things_updated(grid, target_cell, val1, val2, operation, r, c, grid);
    
            if !(*is_disabled) {
                print_grid(*start_x, *start_y, r, c, grid);
            }
    
            return status;
        }
        _ => return 3, // If parsing fails or does not match expected structure
    }    
    1
    // let coord = parser::parse(command, r, c);
    // let coord = parser::validate(command, &r, &c);

    // if let Some((_, Some(ref value))) = coord {
    //     match value {
    //         parser::Value::Oper(left_operand, right_operand, operation) => {
    //             function = *operation as isize;
    //             if function == 13 {
    //                 if let parser::Value::Cell(row, col) = **left_operand {
    //                     *start_x = row;
    //                     *start_y = col;
    //                 }
    //             }
    //         }
    //         _ => {}
    //     }
    // } else {
    //     return 3;
    // }
    // match function {
    //     12 => {
    //         *is_disabled = true;
    //         1
    //     }
    //     11 => {
    //         *is_disabled = false;
    //         print_grid(*start_x, *start_y, r, c, grid);
    //         1
    //     }
    //     13 => {
    //         if !(*is_disabled) {
    //             print_grid(*start_x, *start_y, r, c, grid);
    //         }
    //         1
    //     }
    //     _ => {
    //         if let Some(coords) = coord {
    //             // let status = getting_things_updated(function, &coords[0], &coords[1], &coords[2], r, c);
    //             let status = 1;
    //             if !(*is_disabled) {
    //                 print_grid(*start_x, *start_y, r, c, grid);
    //             }
    //             status
    //         } else {
    //             3
    //         }
    //     }
    // }
}

fn process_first(
    x: usize,
    command: &[String],
    is_disabled: &mut bool,
) -> bool {
    if x != 3 {
        return false;
    }
    if !is_number(&command[1]) || !is_number(&command[2]) {
        return false;
    }

    let r = command[1].parse::<usize>().unwrap();
    let c = command[2].parse::<usize>().unwrap();

    if r < 1 || r > MAX_ROW || c < 1 || c > MAX_COLUMN {
        return false;
    }
    true
}

fn main() {
    let mut start_x = 1;
    let mut start_y = 1;
    let mut is_disabled = false;
    let args: Vec<String> = env::args().collect();
    // part of first processing
    if !process_first(
        args.len(),
        &args,
        &mut is_disabled,
    ) {
        return;
    }
    let r = args[1].parse::<usize>().unwrap();
    let c = args[2].parse::<usize>().unwrap();
    let start = Instant::now();
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
