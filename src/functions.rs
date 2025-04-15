// will compute all range based functions - utility for getting_things_updated
// basic structure - will
use crate::graph::Node;
use crate::types::Coordinates;
use std::cmp::{max, min};
// Range based : MAX MIN AVG SUM
pub struct Range {
    pub start: Coordinates,
    pub end: Coordinates,
}

// check ERR flag

// operand : operand type  --> use pattern matching here
// pub enum Operand {
//     Cell(Coordinates),
//     Const(i32),
// }

// // Arithmetic operations : ADD SUB MUL DIV (b/w two operands - cell/value)
// pub struct ArithmeticOp {
//     pub val1: Operand,
//     pub val2: Operand,
// }

// pub enum FunctionType {
//     Range(Range),  // range based functions
//     Arithmetic(ArithmeticOp), // b/w two cells/ints int/cell
//     ConstantAssignment(Operand), // cell/value
//     Sleep(Operand), // sleep : cell/value
// }
// pub struct Function {
//     pub function: Operation,
//     pub function_type: FunctionType,
// }

// parser
#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Cons,
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
    Avg,
    Sum,
    Std,
    Slp,
    EnableOutput,
    DisableOutput,
    Scrollto,
}
#[derive(Debug)]
pub enum Value {
    Cell(usize, usize),
    Const(isize),
    Oper(Box<Value>, Box<Value>, Operation), //value1 and value2, and the operation or command, respectively
}
// parser ends
// handle sleep individually

// range based functions
pub fn max_function(op1: Operand, op2: Operand, grid: &mut Vec<Vec<Node>>) -> Option<i32> {
    match (op1, op2) {
        (Operand::Cell(coord1), Operand::Cell(coord2)) => {
            // Both are cell references
            let mut max_val = std::i32::MIN;
            for i in coord1.row..=coord2.row {
                for j in coord1.col..=coord2.col {
                    if grid[i as usize][j as usize].valid {
                        max_val = max(max_val, grid[i as usize][j as usize].node_value);
                    } else {
                        // debug // handle ERR cases after some more clarity
                        return None;
                    }
                }
            }
            Some(max_val)
        }
        _ => {
            // println!("Error: max_function only supports Cell-Cell operands");
            // this case wont come - Caller duty - debug
            // None can come only if the cell is invalid (ERR)
            None // Return None to indicate an error/invalid operation
        }
    }
}

pub fn min_function(op1: Operand, op2: Operand, grid: &mut Vec<Vec<Node>>) -> Option<i32> {
    match (op1, op2) {
        (Operand::Cell(coord1), Operand::Cell(coord2)) => {
            // Both are cell references
            let mut min_val = std::i32::MAX;
            for i in coord1.row..=coord2.row {
                for j in coord1.col..=coord2.col {
                    if grid[i as usize][j as usize].valid {
                        min_val = min(min_val, grid[i as usize][j as usize].node_value);
                    } else {
                        // debug // handle ERR cases after some more clarity
                        return None;
                    }
                }
            }
            Some(min_val)
        }
        _ => {
            // println!("Error: max_function only supports Cell-Cell operands");
            // this case wont come - Caller duty - debug
            // None can come only if the cell is invalid (ERR)
            None // Return None to indicate an error/invalid operation
        }
    }
}

pub fn avg_function(op1: Operand, op2: Operand, grid: &mut Vec<Vec<Node>>) -> Option<i32> {
    match (op1, op2) {
        (Operand::Cell(coord1), Operand::Cell(coord2)) => {
            let mut sum = 0;
            let mut count = 0;
            for i in coord1.row..=coord2.row {
                for j in coord1.col..=coord2.col {
                    if grid[i as usize][j as usize].valid {
                        sum += grid[i as usize][j as usize].node_value;
                        count += 1;
                    } else {
                        return None;
                    }
                }
            }
            if count == 0 { None } else { Some(sum / count) }
        }
        _ => None,
    }
}

pub fn sum_function(op1: Operand, op2: Operand, grid: &mut Vec<Vec<Node>>) -> Option<i32> {
    match (op1, op2) {
        (Operand::Cell(coord1), Operand::Cell(coord2)) => {
            let mut sum = 0;
            for i in coord1.row..=coord2.row {
                for j in coord1.col..=coord2.col {
                    if grid[i as usize][j as usize].valid {
                        sum += grid[i as usize][j as usize].node_value;
                    } else {
                        return None;
                    }
                }
            }
            Some(sum)
        }
        _ => None,
    }
}

pub fn stdev_function(op1: Operand, op2: Operand, grid: &mut Vec<Vec<Node>>) -> Option<i32> {
    match (op1, op2) {
        (Operand::Cell(coord1), Operand::Cell(coord2)) => {
            let mut values = Vec::new();
            for i in coord1.row..=coord2.row {
                for j in coord1.col..=coord2.col {
                    if grid[i as usize][j as usize].valid {
                        values.push(grid[i as usize][j as usize].node_value as f64);
                    } else {
                        return None;
                    }
                }
            }
            let n = values.len();
            if n == 0 {
                return None;
            }
            let mean: f64 = values.iter().sum::<f64>() / n as f64;
            let variance: f64 =
                values.iter().map(|&x| (x - mean) * (x - mean)).sum::<f64>() / n as f64;
            let stdev = variance.sqrt();
            Some(stdev.round() as i32) // Round and convert to i32 for consistency
        }
        _ => None,
    }
}

// pub fn sleep_function(op1: Operand, op2: Operand, grid: &mut Vec<Vec<Node>>) -> Option<i32> {

// }

// arith operations - tbd
pub fn is_arithmetic(op: Operation) -> bool {
    match op {
        Operation::Add | Operation::Sub | Operation::Mul | Operation::Div => true,
        _ => false,
    }
}