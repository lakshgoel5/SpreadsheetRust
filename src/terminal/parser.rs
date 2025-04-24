use crate::terminal::functions::Operation;
use crate::terminal::functions::Value;
pub fn validate(
    cmd: &str,
    rows: &usize,
    columns: &usize,
) -> Option<(Option<Value>, Option<Value>)> {
    match cmd.trim() {
        "enable_output" => {
            return Some((
                None,
                Some(Value::Oper(
                    Box::new(Value::Const(-1)),
                    Box::new(Value::Const(-1)),
                    Operation::EnableOutput,
                )),
            ));
        }
        "disable_output" => {
            return Some((
                None,
                Some(Value::Oper(
                    Box::new(Value::Const(-1)),
                    Box::new(Value::Const(-1)),
                    Operation::DisableOutput,
                )),
            ));
        }
        _ => {} // Continue with the regular parsing for other commands
    }

    if cmd.trim().starts_with("scroll_to ") {
        let cell_name = cmd.trim()["scroll_to ".len()..].trim();
        let cell = is_cell(cell_name, rows, columns);
        if let Some(cell) = cell {
            return Some((
                Some(cell),
                Some(Value::Oper(
                    Box::new(Value::Const(-1)),
                    Box::new(Value::Const(-1)),
                    Operation::Scrollto,
                )),
            ));
        } else {
            return None;
        }
    }

    let (cell, exp) = cmd.split_once('=')?;

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
                let op1 = (val[..i]).trim().to_string();
                let op2 = &val[i + 1..].trim().to_string();
                let op1 = is_cell_or_const(&op1.to_string(), rows, columns)?;
                let op2 = is_cell_or_const(&op2.to_string(), rows, columns)?;
                match c {
                    '+' => {
                        return Some((
                            cell,
                            Some(Value::Oper(Box::new(op1), Box::new(op2), Operation::Add)),
                        ));
                    }
                    '-' => {
                        return Some((
                            cell,
                            Some(Value::Oper(Box::new(op1), Box::new(op2), Operation::Sub)),
                        ));
                    }
                    '*' => {
                        return Some((
                            cell,
                            Some(Value::Oper(Box::new(op1), Box::new(op2), Operation::Mul)),
                        ));
                    }
                    '/' => {
                        return Some((
                            cell,
                            Some(Value::Oper(Box::new(op1), Box::new(op2), Operation::Div)),
                        ));
                    }
                    _ => {
                        return None;
                    } //This case is not possible, just for compilation
                }
            }
        }

        let val = is_cell_or_const(&val, rows, columns)?; //for the moment, assuming the RHS to be a constant or cell
        return Some((
            cell,
            Some(Value::Oper(
                Box::new(val),
                Box::new(Value::Const(-1)),
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
                    Box::new(val),
                    Box::new(Value::Const(-1)),
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
            return None;
        }
    } else {
        return None;
    }
    match operation {
        "SUM" => Some((
            cell,
            Some(Value::Oper(Box::new(start), Box::new(end), Operation::Sum)),
        )),
        "AVG" => Some((
            cell,
            Some(Value::Oper(Box::new(start), Box::new(end), Operation::Avg)),
        )),
        "STDEV" => Some((
            cell,
            Some(Value::Oper(Box::new(start), Box::new(end), Operation::Std)),
        )),
        "MIN" => Some((
            cell,
            Some(Value::Oper(Box::new(start), Box::new(end), Operation::Min)),
        )),
        "MAX" => Some((
            cell,
            Some(Value::Oper(Box::new(start), Box::new(end), Operation::Max)),
        )),
        _ => {
            Some((cell, None))
        }
    }
}

pub fn is_cell(exp: &str, rows: &usize, columns: &usize) -> Option<Value> {
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
    Some(Value::Cell(col as i32, row as i32))
}

pub fn is_const(exp: &str) -> Option<Value> {
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

pub fn is_cell_or_const(exp: &str, rows: &usize, columns: &usize) -> Option<Value> {
    if let Some(constant) = is_const(exp) {
        Some(constant)
    } else if let Some(cell) = is_cell(exp, rows, columns) {
        return Some(cell);
    } else {
        return None;
    }
}

// assignment - cell, value - done
// basic math - cells, values
// sleep - cell, value - done
// range operations - cells, values - done
