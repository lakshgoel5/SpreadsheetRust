#![allow(dead_code)]
/// Module responsible for validating and parsing commands given to the spreadsheet.
///
/// This includes:
/// - Recognizing valid cell identifiers
/// - Parsing constants and expressions
/// - Constructing a `Value` AST representing operations or values
/// - Supporting arithmetic, sleep, and range-based operations like SUM, AVG, etc.
use crate::common::Operation;
use crate::common::Value;

fn is_cell(exp: &str, columns: &usize, rows: &usize) -> Option<Value> {
    let mut col = 0;
    let mut row = 0;

    let chars: Vec<char> = exp.chars().collect();
    let mut i = 0;
    while i < 3 {
        if chars[i].is_alphabetic() {
            col = col * 26 + (chars[i] as u8 - b'A') as usize + 1;
        } else {
            break;
        }
        i += 1;
    }
    if exp.chars().count() - i > 3 || i == 0 {
        return None;
    }
    while i < exp.chars().count() {
        if chars[i].is_numeric() {
            row = row * 10 + (chars[i] as u8 - b'0') as usize;
        } else {
            return None;
        }
        i += 1;
    }
    if row > *rows || col > *columns {
        return None;
    }
    Some(Value::Cell(row, col))
}

fn is_const(exp: &str) -> Option<Value> {
    // let mut ans = 0;
    // for c in exp.chars() {
    //     if c.is_numeric() {
    //         ans = ans*10 + (c as u8 - '0' as u8) as usize;
    //     } else {
    //         return None;
    //     }
    // }
    // return Some(Value::Const(ans));
    match exp.parse::<isize>() {
        Ok(ans) => Some(Value::Const(ans)),
        Err(_) => None,
    }
}

fn is_cell_or_const(exp: &str, rows: &usize, columns: &usize) -> Option<Value> {
    if let Some(constant) = is_const(exp) {
        Some(constant)
    } else if let Some(cell) = is_cell(exp, rows, columns) {
        return Some(cell);
    } else {
        return None;
    }
}

pub fn validate(
    cmd: &str,
    rows: &usize,
    columns: &usize,
) -> Option<(Option<Value>, Option<Value>)> {
    match cmd.trim() {
        "undo" => return Some((None, Some(Value::Oper(None, None, Operation::Undo)))),
        "redo" => return Some((None, Some(Value::Oper(None, None, Operation::Redo)))),
        "enable_output" => {
            return Some((None, Some(Value::Oper(None, None, Operation::EnableOutput))));
        }
        "disable_output" => {
            return Some((
                None,
                Some(Value::Oper(None, None, Operation::DisableOutput)),
            ));
        }
        "w" => {
            return Some((None, Some(Value::Oper(None, None, Operation::Up))));
        }
        "s" => {
            return Some((None, Some(Value::Oper(None, None, Operation::Down))));
        }
        "a" => {
            return Some((None, Some(Value::Oper(None, None, Operation::Left))));
        }
        "d" => {
            return Some((None, Some(Value::Oper(None, None, Operation::Right))));
        }
        "q" => {
            return Some((None, Some(Value::Oper(None, None, Operation::Quit))));
        }
        _ => {} // Continue with the regular parsing for other commands
    }

    if cmd.trim().starts_with("scroll_to ") {
        let cell_name = cmd.trim()["scroll_to ".len()..].trim().to_string();
        let cell = is_cell(&cell_name, rows, columns);
        if let Some(cell) = cell {
            return Some((
                Some(cell),
                Some(Value::Oper(None, None, Operation::ScrollTo)),
            ));
        } else {
            // println!("Invalid cell name");
            return None;
        }
    }

    if cmd.trim().starts_with("save ") {
        let file_name = cmd.trim()["save ".len()..].trim().to_string();
        return Some((
            None,
            Some(Value::Oper(None, None, Operation::Save(file_name))),
        ));
    }

    if cmd.trim().starts_with("web ") {
        let file_name = cmd.trim()["web ".len()..].trim().to_string();
        return Some((
            None,
            Some(Value::Oper(None, None, Operation::Web(file_name))),
        ));
    }

    let Some((cell, exp)) = cmd.split_once('=') else {
        // eprintln!("Could not find a valid exp being assigned to a valid cell");
        return None;
    };

    let cell = String::from(cell).trim().to_string();
    let cell = is_cell(&cell, rows, columns);

    let Some((operation, range)) = exp.split_once('(') else {
        // basic math operations or constant (0-4)
        let val = (String::from(exp)).trim().to_string();
        let operators = ["+", "-", "*", "/"];
        for (i, c) in val.chars().enumerate() {
            if i == 0 && c == '-' {
                continue;
            }
            if operators.contains(&c.to_string().as_str()) {
                let op1_str = val[..i].trim();
                let op2_str = val[i + 1..].trim();
                let op1 = is_cell_or_const(op1_str, rows, columns)?;
                let op2 = is_cell_or_const(op2_str, rows, columns)?;

                match c {
                    '+' => {
                        return Some((
                            cell,
                            Some(Value::Oper(
                                Some(Box::new(op1)),
                                Some(Box::new(op2)),
                                Operation::Add,
                            )),
                        ));
                    }
                    '-' => {
                        return Some((
                            cell,
                            Some(Value::Oper(
                                Some(Box::new(op1)),
                                Some(Box::new(op2)),
                                Operation::Sub,
                            )),
                        ));
                    }
                    '*' => {
                        return Some((
                            cell,
                            Some(Value::Oper(
                                Some(Box::new(op1)),
                                Some(Box::new(op2)),
                                Operation::Mul,
                            )),
                        ));
                    }
                    '/' => {
                        return Some((
                            cell,
                            Some(Value::Oper(
                                Some(Box::new(op1)),
                                Some(Box::new(op2)),
                                Operation::Div,
                            )),
                        ));
                    }
                    _ => {
                        // eprintln!("Invalid operation");
                        return None;
                    } //This case is not possible, just for compilation
                }
            }
        }

        let val = is_cell_or_const(&val, rows, columns)?; //for the moment, assuming the RHS to be a constant or cell
        // println!("{} {}", cmd, cmd); //---------------debugger
        return Some((
            cell,
            Some(Value::Oper(
                Some(Box::new(val)),
                Some(Box::new(Value::Const(0))),
                Operation::Cons,
            )),
        ));
    };
    let range = range.strip_suffix(')')?.to_string(); //removing the closing bracket

    let Some((start, end)) = range.split_once(':') else {
        // SLEEP (the keyword 'SLEEP' is not checked for, it is taken fro granted)
        let val = range;
        let val = is_cell_or_const(&val, rows, columns);
        if let Some(val) = val {
            return Some((
                cell,
                Some(Value::Oper(
                    Some(Box::new(val)),
                    Some(Box::new(Value::Const(0))),
                    Operation::Slp,
                )),
            ));
        }
        return Some((cell, None));
    };
    let start = String::from(start);
    let end = String::from(end);
    let start = is_cell(&start, rows, columns)?;
    let end = is_cell(&end, rows, columns)?;
    if let (Value::Cell(r, c), Value::Cell(r2, c2)) = (&start, &end) {
        if r > r2 || c > c2 {
            // eprintln!("Invalid range, start is greater than end");
            return None;
        }
    } else {
        return None;
    }
    match operation {
        "SUM" => Some((
            cell,
            Some(Value::Oper(
                Some(Box::new(start)),
                Some(Box::new(end)),
                Operation::Sum,
            )),
        )),
        "AVG" => Some((
            cell,
            Some(Value::Oper(
                Some(Box::new(start)),
                Some(Box::new(end)),
                Operation::Avg,
            )),
        )),
        "STDEV" => Some((
            cell,
            Some(Value::Oper(
                Some(Box::new(start)),
                Some(Box::new(end)),
                Operation::Std,
            )),
        )),
        "MIN" => Some((
            cell,
            Some(Value::Oper(
                Some(Box::new(start)),
                Some(Box::new(end)),
                Operation::Min,
            )),
        )),
        "MAX" => Some((
            cell,
            Some(Value::Oper(
                Some(Box::new(start)),
                Some(Box::new(end)),
                Operation::Max,
            )),
        )),
        _ => {
            // eprintln!("Invalid operation");
            Some((cell, None))
        }
    }
}

// assignment - cell, value - done
// basic math - cells, values
// sleep - cell, value - done
// range operations - cells, values - done
