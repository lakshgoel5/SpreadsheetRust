/// Spreadsheet module for the terminal-based interface.
///
/// This module provides functionality for rendering the spreadsheet grid in 
/// the terminal, handling user commands, and displaying the current state of 
/// cells to the user. It acts as the frontend for the terminal-based version 
/// of the spreadsheet application.
use crate::terminal::backend::getting_things_updated;
use crate::terminal::functions::Value;
use crate::terminal::graph::Node;
use crate::terminal::parser;
use crate::terminal::functions::Operation;
use crate::terminal::types::Coordinates;
use std::cmp;
use std::io;
use std::io::Write;

/// Constants defining the maximum dimensions of the spreadsheet
const MAX_ROW: usize = 999;
const MAX_COLUMN: usize = 18278;

/// Translates a column number to its Excel-style letter representation (A, B, C, ..., AA, AB, etc.).
///
/// # Arguments
///
/// * `y` - The 1-based column index to convert
///
/// # Returns
///
/// A string containing the column letter(s)
pub fn column_decoder(mut j: usize) -> String {
    let mut cc = Vec::new();
    while j > 0 {
        j -= 1;
        cc.push((b'A' + (j % 26) as u8) as char);
        j /= 26;
    }
    cc.reverse();
    cc.into_iter().collect()
}

/// Prints the current state of the spreadsheet grid to the terminal.
///
/// # Arguments
///
/// * `start_x` - The starting row index for display
/// * `start_y` - The starting column index for display 
/// * `r` - The number of rows in the grid
/// * `c` - The number of columns in the grid
/// * `grid` - The spreadsheet grid containing the cells
pub fn print_grid(start_x: usize, start_y: usize, r: usize, c: usize, grid: &mut [Vec<Node>]) {
    let max_x = cmp::min(9 + start_x, r);
    let max_y = cmp::min(9 + start_y, c);

    for (i, _row) in grid.iter().enumerate().take(max_x + 1).skip(start_x - 1) {
        for j in start_y - 1..=max_y {
            if i == start_x - 1 && j == start_y - 1 {
                print!("{:>12}", " ");
            } else if i == start_x - 1 && j != start_y - 1 {
                print!("{:>12}", column_decoder(j));
            } else if j == start_y - 1 {
                print!("{:>12}", i);
            } else if grid[i][j].valid {
                print!("{:>12}", grid[i][j].node_value);
            } else {
                print!("{:>12}", "ERR");
            }
        }
        println!();
    }
}

/// Displays a status message with execution time.
///
/// # Arguments
///
/// * `status` - The status code of the previous operation
/// * `time` - The execution time of the operation in seconds
pub fn display_status(x: i32, time_taken: f64) {
    print!("[{:.2}] ", time_taken);
    match x {
        1 => print!("(ok) > "),                 // relevant
        2 => print!("(invalid range) > "), // not relevant to autograder - will have to change parser if want to // debug
        3 => print!("(unrecognized cmd) > "), // relevant
        4 => print!("(invalid row/column) > "), // ig not relevant
        5 => print!("(cycle not allowed) > "), // relevant
        _ => (),
    }
    io::stdout().flush().unwrap();
}

fn is_number(str: &str) -> bool {
    !str.is_empty() && str.chars().all(|c| c.is_ascii_digit())
}

/// Processes a single command and updates the spreadsheet state accordingly.
///
/// # Arguments
///
/// * `cmd` - The command string to process
/// * `start_x` - The current row index for display (may be updated)
/// * `start_y` - The current column index for display (may be updated)
/// * `r` - The number of rows in the grid
/// * `c` - The number of columns in the grid
/// * `is_disabled` - Whether output is disabled
/// * `grid` - The spreadsheet grid to update
///
/// # Returns
///
/// An integer status code indicating success or specific failure modes
#[allow(unreachable_code)]
pub fn process_command(
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

    // debug - complete this
    match parser::validate(command, &r, &c) {
        Some((Some(Value::Cell(col, row)), Some(Value::Oper(v1, v2, op)))) => {
            // Handle special operations
            if op == Operation::Scrollto {
                {
                    *start_x = row as usize;
                    *start_y = col as usize;
                    if !(*is_disabled) {
                        print_grid(*start_x, *start_y, r, c, grid);
                    }
                    return 1;
                }
                
            }

            let target_cell = Coordinates { row, col };
            let (value1, value2) = match (&*v1, &*v2) {
                (Value::Cell(c1, r1), Value::Cell(c2, r2)) => (
                    Coordinates { row: *r1, col: *c1 },
                    Coordinates { row: *r2, col: *c2 },
                ),
                (Value::Cell(c1, r1), Value::Const(val)) => (
                    Coordinates { row: *r1, col: *c1 },
                    Coordinates {
                        row: *val as i32,
                        col: -1,
                    },
                ),
                (Value::Const(val), Value::Cell(c2, r2)) => (
                    Coordinates {
                        row: *val as i32,
                        col: -1,
                    },
                    Coordinates { row: *r2, col: *c2 },
                ),
                (Value::Const(val1), Value::Const(val2)) => (
                    Coordinates {
                        row: *val1 as i32,
                        col: -1,
                    },
                    Coordinates {
                        row: *val2 as i32,
                        col: -1,
                    },
                ),
                _ => {
                    if !(*is_disabled) {
                        print_grid(*start_x, *start_y, r, c, grid);
                    }
                    return 3; // Invalid operands
                }
            };
            let operation = op;

            let status = getting_things_updated(grid, target_cell, value1, value2, operation);

            if !(*is_disabled) {
                print_grid(*start_x, *start_y, r, c, grid);
            }

            return status;
        }
        Some((None, Some(Value::Oper(_v1, _v2, op)))) => {
            // Handle special operations
            match op {
                Operation::EnableOutput => {
                    *is_disabled = false;
                    print_grid(*start_x, *start_y, r, c, grid);
                    return 1;
                }
                Operation::DisableOutput => {
                    *is_disabled = true;
                    return 1;
                }
                _ => {
                    if !(*is_disabled) {
                        print_grid(*start_x, *start_y, r, c, grid);
                    }
                    return 3; // Invalid operands
                }
            }
        }
        _ => {
            if !(*is_disabled) {
                print_grid(*start_x, *start_y, r, c, grid);
            }
            return 3; // Invalid operands
        }
    }
    1
}

/// Processes the command line arguments provided to the application.
///
/// # Arguments
///
/// * `argc` - The number of arguments
/// * `args` - The vector of argument strings
/// * `is_disabled` - Whether the output should be disabled
///
/// # Returns
///
/// `true` if arguments are valid and processing should continue, `false` otherwise
pub fn process_first(x: usize, command: &[String], _is_disabled: &mut bool) -> bool {
    if x != 3 {
        return false;
    }
    if !is_number(&command[1]) || !is_number(&command[2]) {
        return false;
    }

    let r = command[1].parse::<usize>().unwrap();
    let c = command[2].parse::<usize>().unwrap();

    if !(1..=MAX_ROW).contains(&r) || !(1..=MAX_COLUMN).contains(&c) {
        return false;
    }
    true
}
