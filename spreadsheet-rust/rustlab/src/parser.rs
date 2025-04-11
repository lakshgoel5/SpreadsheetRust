pub fn validate(cmd: &String, rows: &usize, columns: &usize) -> Option<(Option<Value>, Option<Value>)> {
    let Some((cell, exp)) = cmd.split_once('=') else {
        eprintln!("Could not find a valid exp being assigned to a valid cell");
        return None;
    };

    let cell = String::from(cell).trim().to_string();
    let cell = is_cell(&cell, rows, columns);

    let Some((operation, range)) = exp.split_once('(') else {
        // basic math operations or constant (0-4)
        let val = (String::from(exp)).trim().to_string();
        let operators = vec!["+", "-", "*", "/"];
        for (i, c) in val.chars().enumerate() {
            if i==0 && c=='-' { continue; }
            if operators.contains(&c.to_string().as_str()) {
                let op1 = (&val[..i]).trim().to_string();
                let op2 = &val[i+1..].trim().to_string();
                let op1 = is_cell_or_const(&op1.to_string(), rows, columns)?;
                let op2 = is_cell_or_const(&op2.to_string(), rows, columns)?;
                match c {
                    '+' => return Some((cell, Some(Value::Oper(Box::new(op1), Box::new(op2), Operation::ADD)))),
                    '-' => return Some((cell, Some(Value::Oper(Box::new(op1), Box::new(op2), Operation::SUB)))),
                    '*' => return Some((cell, Some(Value::Oper(Box::new(op1), Box::new(op2), Operation::MUL)))),
                    '/' => return Some((cell, Some(Value::Oper(Box::new(op1), Box::new(op2), Operation::DIV)))),
                    _ => { eprintln!("Invalid operation"); return None; } //This case is not possible, just for compilation
                }
            }
        }

        let val= is_cell_or_const(&val, rows, columns)?; //for the moment, assuming the RHS to be a constant or cell
        // println!("{} {}", cmd, cmd); //---------------debugger
        return Some((cell, Some(Value::Oper(Box::new(val), Box::new(Value::Const(0)), Operation::CONS))));
    };
    let range = range.strip_suffix(')')?.to_string(); //removing the closing bracket

    let Some((start, end)) = range.split_once(':') else {
        // SLEEP (the keyword 'SLEEP' is not checked for, it is taken fro granted)
        let val = String::from(range);
        let val = is_cell_or_const(&val, rows, columns);
        if let Some(val) = val {
            return Some((cell, Some(Value::Oper(Box::new(val), Box::new(Value::Const(0)), Operation::SLP))));
        }
        return Some((cell, None));
    };
    let start = String::from(start);
    let end = String::from(end);
    let start = is_cell(&start, rows, columns)?;
    let end = is_cell(&end, rows, columns)?;
    if let (Value::Cell(r, c), Value::Cell(r2, c2)) = (&start, &end) {
        if r>r2 || c>c2 {
            eprintln!("Invalid range, start is greater than end");
            return None;
        }
    } else {
        return None;
    }
    match operation {
        "SUM" => return Some((cell, Some(Value::Oper(Box::new(start), Box::new(end), Operation::SUM)))),
        "AVG" => return Some((cell, Some(Value::Oper(Box::new(start), Box::new(end), Operation::AVG)))),
        "STDEV" => return Some((cell, Some(Value::Oper(Box::new(start), Box::new(end), Operation::STD)))),
        "MIN" => return Some((cell, Some(Value::Oper(Box::new(start), Box::new(end), Operation::MIN)))),
        "MAX" => return Some((cell, Some(Value::Oper(Box::new(start), Box::new(end), Operation::MAX)))),
        _ => { eprintln!("Invalid operation");  return Some((cell, None)); }
    }
}
#[derive(Debug)]
enum Operation {
    CONS = 0,
    ADD = 1,
    SUB = 2,
    MUL = 3,
    DIV = 4,
    MIN = 5,
    MAX = 6,
    AVG = 7,
    SUM = 8,
    STD = 9,
    SLP = 10
}
#[derive(Debug)]
pub enum Value {
    Cell(usize, usize),
    Const(isize),
    Oper(Box<Value>, Box<Value>, Operation) //value1 and value2, and the operation or command, respectively
}

pub fn is_cell(exp: &String, rows: &usize, columns: &usize) -> Option<Value> {
    let mut col = 0;
    let mut row = 0;
    
    let chars : Vec<char> = exp.chars().collect();
    let mut i = 0;
    while i < 3 {
        if chars[i].is_alphabetic() {
            col = col * 26 + (chars[i] as u8 - 'A' as u8) as usize + 1;
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
            row = row * 10 + (chars[i] as u8 - '0' as u8) as usize;
        } else {
            return None;
        }
        i += 1;
    }
    if row > *rows || col > *columns {
        return None;
    }
    return Some(Value::Cell(col, row));
}

pub fn is_const(exp: &String) -> Option<Value> {
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

pub fn is_cell_or_const(exp: &String, rows: &usize, columns: &usize) -> Option<Value> {
    if let Some(constant) = is_const(exp) {
        return Some(constant);
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