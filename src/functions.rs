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
pub fn max_function(value1: Coordinates, value2: Coordinates, grid: &mut Vec<Vec<Node>>) -> Option<i32> {
    // Both are cell references
    let mut max_val = std::i32::MIN;
    // bool flag = false;
    for i in value1.row..=value2.row {
        for j in value1.col..=value2.col {
            if grid[i as usize][j as usize].valid {
                max_val = max(max_val, grid[i as usize][j as usize].node_value);
            } else {
                // debug // handle ERR cases after some more clarity
                // flag = true;
                return None;
            }
        }
    }
    Some(max_val)
}

pub fn min_function(value1: Coordinates, value2: Coordinates, grid: &mut Vec<Vec<Node>>) -> Option<i32> {
    // Both are cell references
    let mut min_val = std::i32::MAX;
    // bool flag = false;
    for i in value1.row..=value2.row {
        for j in value1.col..=value2.col {
            if grid[i as usize][j as usize].valid {
                min_val = min(min_val, grid[i as usize][j as usize].node_value);
            } else {
                // debug // handle ERR cases after some more clarity
                // flag = true;
                return None;
            }
        }
    }
    Some(min_val)
}

pub fn avg_function(value1: Coordinates, value2: Coordinates, grid: &mut Vec<Vec<Node>>) -> Option<i32> {
    let mut sum = 0;
    let mut count = 0;
    for i in value1.row..=value2.row {
        for j in value1.col..=value2.col {
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

pub fn sum_function(value1: Coordinates, value2: Coordinates, grid: &mut Vec<Vec<Node>>) -> Option<i32> {
    let mut sum = 0;
    for i in value1.row..=value2.row {
        for j in value1.col..=value2.col {
            if grid[i as usize][j as usize].valid {
                sum += grid[i as usize][j as usize].node_value;
            } else {
                return None;
            }
        }
    }
    Some(sum)
}

pub fn stdev_function(value1: Coordinates, value2: Coordinates, grid: &mut Vec<Vec<Node>>) -> Option<i32> {
    let mut sum = 0f64;
    let mut count = 0;
    
    // First pass: calculate sum and count
    for i in value1.row..=value2.row {
        for j in value1.col..=value2.col {
            let node = &grid[i as usize][j as usize];
            if node.valid {
                sum += node.node_value as f64;
                count += 1;
            }
            else{
                return None;
            }
        }
    }

    if count == 0 {
        return Some(0); // Consistent with C behavior
    }

    let mean = sum / count as f64;

    // Second pass: calculate variance
    let mut stdev = 0f64;
    for i in value1.row..=value2.row {
        for j in value1.col..=value2.col {
            let node = &grid[i as usize][j as usize];
            //
            if !node.valid {
                return None; // In C, this sets the target to invalid
            }
            let val = node.node_value as f64;
            stdev += (val - mean) * (val - mean);
        }
    }

    let result = (stdev / count as f64).sqrt().round() as i32;
    Some(result)
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