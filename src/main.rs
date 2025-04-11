use std::env;
mod parser;

fn main() {
    // decoding rows and columns
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <rows> <columns>", args[0]);
        std::process::exit(1);
    }
    let rows: usize = args[1].parse().expect("Invalid number of rows");
    let columns: usize = args[2].parse().expect("Invalid number of columns");
    if rows>999 || columns>18278 {
        eprintln! ("Invalid input: rows and cols need to be within 999 and ZZZ respectively");
        std::process::exit(1);
    }

    // reading command and replacing the trailing newline with null character
    let mut cmd = String::new();
    let _bytes_read = std::io::stdin().read_line(&mut cmd).expect("Failed to read command");
    let cmd = String::from(cmd.trim());

    // calling parser
    let cell = parser::validate(&cmd, &rows, &columns);
    if let Some(c) = cell {
        println!("{:?}", c);
    } else {
        eprintln!("Invalid command");
    }
}