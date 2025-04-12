use std::env;
use std::io::{self, BufRead, Write};
use std::time::{Instant};
use std::cmp;
use std::ffi::CString;
use std::ptr;
mod parser;

const MAX_ROW: usize = 999;
const MAX_COLUMN: usize = 18278;


static mut IS_DISABLED: bool = false;

static mut GRID: Option<Vec<Vec<i32>>> = None;

fn generate_grid(r: usize, c: usize) {
    unsafe {
        GRID = Some(vec![vec![0; c + 1]; r + 1]);
    }
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

// , graph: &Graph
fn print_grid(start_x: usize, start_y: usize, r: usize, c: usize) {
    let max_x = cmp::min(9 + start_x, r);
    let max_y = cmp::min(9 + start_y, c);

    for i in start_x - 1..=max_x {
        for j in start_y - 1..=max_y {
            if i == start_x - 1 && j == start_y - 1 {
                print!("{:>12}", " ");
            } 
            else if i == start_x - 1 && j != start_y - 1 {
                print!("{:>12}", column_decoder(j));
            } 
            else if j == start_y - 1 {
                print!("{:>12}", i);
            } 
            else {
                unsafe {
                    print!("{:>12}", GRID.as_ref().unwrap()[i][j]);
                }
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
fn process_command(command: &str, start_x: &mut usize, start_y: &mut usize, r: usize, c: usize) -> i32 {
    match command {
        "q" => return 0,
        "w" => {
            *start_x = if *start_x > 10 { *start_x - 10 } else { 1 };
            unsafe {
                if !IS_DISABLED {
                    print_grid(*start_x, *start_y, r, c);
                }
            }
            return 1;
        }
        "s" => {
            if *start_x + 10 <= r {
                *start_x += 10;
                *start_x = cmp::min(*start_x, r - 9);
            }
            unsafe {
                if !IS_DISABLED {
                    print_grid(*start_x, *start_y, r, c);
                }
            }
            return 1;
        }
        "a" => {
            *start_y = if *start_y > 10 { *start_y - 10 } else { 1 };
            unsafe {
                if !IS_DISABLED {
                    print_grid(*start_x, *start_y, r, c);
                }
            }
            return 1;
        }
        "d" => {
            if *start_y + 10 <= c {
                *start_y += 10;
                *start_y = cmp::min(*start_y, c - 9);
            }
            unsafe {
                if !IS_DISABLED {
                    print_grid(*start_x, *start_y, r, c);
                }
            }
            return 1;
        }
        _ => {}
    }

    // let mut function: isize = -1;

    // let coord = parser::validate(command, &r, &c);

    // if let Some((_, Some(value))) = coord {
    //     match value {
    //         parser::Value::Oper(left_operand, right_operand, operation) => {
    //             function = operation as isize;

    //             // match operation {
    //             //     parser::Operation::Add => function = 1,
    //             //     parser::Operation::Sub => function = 2,
    //             //     parser::Operation::Mul => function = 3,
    //             //     parser::Operation::Div => function = 4,
    //             //     parser::Operation::Sum => function = 5,
    //             //     parser::Operation::Avg => function = 6,
    //             //     parser::Operation::Std => function = 7,
    //             //     parser::Operation::Min => function = 8,
    //             //     parser::Operation::Max => function = 9,
    //             //     parser::Operation::Slp => function = 10,
    //             //     _ => {}
    //             // }
    //         }
    //     }
    // } 
    // else {
    //     return 3;
    // }
    return 1;
    // match function {
    //     12 => {
    //         unsafe { IS_DISABLED = true; }
    //         1
    //     }
    //     11 => {
    //         unsafe { IS_DISABLED = false; }
    //         print_grid(*start_x, *start_y, r, c);
    //         1
    //     }
    //     13 => {
    //         if let Some(coords) = coord {
    //             *start_x = coords[0].row;
    //             *start_y = coords[0].col;
    //             unsafe {
    //                 if !IS_DISABLED {
    //                     print_grid(*start_x, *start_y, r, c);
    //                 }
    //             }
    //         }
    //         1
    //     }
    //     -1 => {
    //         unsafe {
    //             if !IS_DISABLED {
    //                 print_grid(*start_x, *start_y, r, c);
    //             }
    //         }
    //         3
    //     }
    //     _ => {
    //         if let Some(coords) = coord {
    //             // let status = getting_things_updated(function, &coords[0], &coords[1], &coords[2], r, c);
    //             let status = 1;
    //             unsafe {
    //                 if !IS_DISABLED {
    //                     print_grid(*start_x, *start_y, r, c);
    //                 }
    //             }
    //             status
    //         } else {
    //             3
    //         }
    //     }
    // }
}

fn process_first(x: usize, command: &[String], start_x: &mut usize, start_y: &mut usize) -> bool {
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

    let start = Instant::now();
    unsafe {
        // let graph = create_graph(r + 1, c + 1);
        generate_grid(r, c);
        if !IS_DISABLED {
            print_grid(*start_x, *start_y, r, c);
        }
    }
    let duration = start.elapsed();
    display_status(1, duration.as_secs_f64());
    true
}

fn main() {
    let mut start_x = 1;
    let mut start_y = 1;

    let args: Vec<String> = env::args().collect();

    if !process_first(args.len(), &args, &mut start_x, &mut start_y) {
        return;
    }

    let r = args[1].parse::<usize>().unwrap();
    let c = args[2].parse::<usize>().unwrap();

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
            println!("[0.0] (unrecognized cmd) > ");
            // print_grid(start_x, start_y, r, c, &graph);
            continue;
        }

        let start = Instant::now();
        // , &graph // debug
        let status = process_command(&command, &mut start_x, &mut start_y, r, c);
        let duration = start.elapsed();

        // quit status
        if status == 0 {
            break;
        }
        display_status(status, duration.as_secs_f64());
    }
}
