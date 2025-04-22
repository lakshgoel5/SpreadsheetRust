#![allow(dead_code)]
use crate::common::Value;
use std::cmp;
use std::io;
use std::io::Write;
use std::process::Command;

//init_frontend(r, c) -> init_backend(r, c), Print_grid(), run_counter(): returns void
//print grid() -> get_value(value::cell) : returns void
//run_counter -> while loop for argument, process_command(r,c, string), Print_grid() : return void
//display_status
use crate::backend::backend::*;
use std::time::{Duration, Instant};

pub struct Frontend {
    start: Value,
    dimension: Value,
    backend: Backend,
    print_enabled: bool,
}

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

impl Frontend {
    pub fn print_grid(&self) {
        if !self.print_enabled {
            return;
        }
        let location = self.start.clone();
        let dimension = self.dimension.clone();
        if let (Value::Cell(start_x, start_y), Value::Cell(rows, cols)) = (location, dimension) {
            let max_x = cmp::min(9 + start_x, rows);
            let max_y = cmp::min(9 + start_y, cols);
            for i in start_x - 1..=max_x {
                for j in start_y - 1..=max_y {
                    if i == start_x - 1 && j == start_y - 1 {
                        print!("{:>12}", " ");
                    } else if i == start_x - 1 {
                        print!("{:>12}", column_decoder(j));
                    } else if j == start_y - 1 {
                        print!("{:>12}", i);
                    } else {
                        match self.backend.get_grid().get_node_value(i, j) {
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

    pub fn init_frontend(rows: usize, columns: usize, path: &str) -> Self {
        if path == "" {
            let backend = Backend::init_backend(rows, columns);
            Frontend {
                start: Value::Cell(1, 1),
                dimension: Value::Cell(rows, columns),
                backend,
                print_enabled: true,
            }
        } else {
            let backend = match Backend::deserial(path) {
                Ok(backend) => backend,
                Err(e) => {
                    eprintln!("Failed to deserialize backend: {}", e);
                    Backend::init_backend(rows, columns)
                }
            };
            let rows = backend.get_grid().get_row_size() - 1;
            let columns = backend.get_grid().get_column_size() - 1;
            println!("{}", columns);
            Frontend {
                start: Value::Cell(1, 1),
                dimension: Value::Cell(rows, columns),
                backend,
                print_enabled: true,
            }
        }
    }

    pub fn run_frontend(&mut self) {
        self.display(Status::Success, Duration::from_secs(0).as_secs_f64());
        self.run_counter();
    }

    fn execute_status(&mut self, status: &Status) {
        match status {
            Status::Left => {
                if self.start.col() > 10 {
                    self.start.assign_col(self.start.col() - 10);
                } else {
                    self.start.assign_col(1);
                }
            }
            Status::Right => {
                if (self.start.col() as isize) < (self.dimension.col() as isize) - 10 {
                    self.start
                        .assign_col(cmp::min(self.start.col() + 10, self.dimension.col() - 9));
                }
            }
            Status::Up => {
                if self.start.row() > 10 {
                    self.start.assign_row(self.start.row() - 10);
                } else {
                    self.start.assign_row(1);
                }
            }
            Status::Down => {
                if (self.start.row() as isize) < (self.dimension.row() as isize) - 10 {
                    self.start
                        .assign_row(cmp::min(self.start.row() + 10, self.dimension.row() - 9));
                }
            }
            Status::PrintDisabled => {
                self.print_enabled = false;
            }
            Status::PrintEnabled => {
                self.print_enabled = true;
            }
            Status::ScrollTo(row, col) => {
                self.start.assign_row(*row);
                self.start.assign_col(*col);
            }
            Status::Web => {
                Command::new("trunk")
                    .arg("serve")
                    .arg("--open")
                    .spawn()
                    .expect("Failed to start trunk")
                    .wait()
                    .expect("Failed to wait for trunk process");
            }
            _ => (),
        }
    }

    pub fn display(&self, status: Status, elapsed_time: f64) {
        self.print_grid();
        match status {
            Status::Success => print!("[{:.2}] (ok) > ", elapsed_time),
            Status::InvalidRange => print!("[{:.2}] (invalid range) > ", elapsed_time),
            Status::UnrecognizedCmd => print!("[{:.2}] (unrecognized command) > ", elapsed_time),
            Status::InvalidRowColumn => print!("[{:.2}] (invalid row or column) > ", elapsed_time),
            Status::CircularDependency => print!("[{:.2}] (cycle not allowed) > ", elapsed_time),
            Status::PrintEnabled => print!("[{:.2}] (ok) > ", elapsed_time),
            Status::PrintDisabled => print!("[{:.2}] (ok) > ", elapsed_time),
            Status::ScrollTo(_, _) => print!("[{:.2}] (ok) > ", elapsed_time),
            Status::Up => print!("[{:.2}] (ok) > ", elapsed_time),
            Status::Down => print!("[{:.2}] (ok) > ", elapsed_time),
            Status::Left => print!("[{:.2}] (ok) > ", elapsed_time),
            Status::Right => print!("[{:.2}] (ok) > ", elapsed_time),
            _ => (),
        }
        io::stdout().flush().unwrap();
    }

    pub fn run_counter(&mut self) {
        let mut input = String::new();
        let stdin = std::io::stdin();

        loop {
            input.clear();

            if stdin.read_line(&mut input).is_err() {
                self.display(
                    Status::UnrecognizedCmd,
                    Duration::from_secs(0).as_secs_f64(),
                );
                continue;
            }
            let start_time = Instant::now();
            let command = input.trim().to_string();
            // let status = Status::Success;
            // if command == ("save".to_string()) {
            //     self.backend.serial("tester.json").expect("Failed to save file");

            // } else {
            let status =
                self.backend
                    .process_command(self.dimension.row(), self.dimension.col(), command);
            // }
            if status == Status::Quit {
                break;
            }
            self.execute_status(&status);
            let elapsed_time = start_time.elapsed();
            self.display(status, elapsed_time.as_secs_f64());
        }
    }
}
