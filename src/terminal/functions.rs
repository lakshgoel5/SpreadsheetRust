/// Module for spreadsheet functions implementation.
///
/// Provides implementations of various cell operations including:
/// - Range-based operations (SUM, MIN, MAX, AVG, STDEV)
/// - Arithmetic operations (Add, Sub, Mul, Div)
/// - Special operations like Sleep and Constant assignment
/// 
/// These functions are the core computational elements of the spreadsheet.
use crate::terminal::graph::Node;
use crate::terminal::types::Coordinates;
use std::cmp::{max, min};

/// Represents a range of cells in the spreadsheet.
///
/// Used for range-based functions like SUM, MIN, MAX, AVG, and STDEV
/// to specify the corners of a rectangular selection of cells.
pub struct Range {
    /// The starting cell (top-left corner) of the range
    pub start: Coordinates,
    
    /// The ending cell (bottom-right corner) of the range
    pub end: Coordinates,
}

/// Supported operations in the spreadsheet.
///
/// This enum defines all possible operations that can be performed
/// on cells in the spreadsheet, including both arithmetic operations
/// and range-based functions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operation {
    /// Assign a constant value to a cell
    Cons,
    
    /// Add two values
    Add,
    
    /// Subtract one value from another
    Sub,
    
    /// Multiply two values
    Mul,
    
    /// Divide one value by another
    Div,
    
    /// Find the minimum value in a range
    Min,
    
    /// Find the maximum value in a range
    Max,
    
    /// Calculate the average of values in a range
    Avg,
    
    /// Calculate the sum of values in a range
    Sum,
    
    /// Calculate the standard deviation of values in a range
    Std,
    
    /// Sleep operation (pause execution)
    Slp,
    
    /// Enable output to the terminal
    EnableOutput,
    
    /// Disable output to the terminal
    DisableOutput,
    
    /// Scroll to a specific cell
    Scrollto,
}

impl Default for Operation {
    fn default() -> Self {
        Operation::Cons
    }
}

/// Represents a value in the spreadsheet expression system.
///
/// Values can be cell references, constants, or operations involving
/// other values.
#[derive(Debug)]
pub enum Value {
    /// A cell reference with row and column indices
    Cell(i32, i32),
    
    /// A constant integer value
    Const(isize),
    
    /// An operation involving two values and an operator
    Oper(Box<Value>, Box<Value>, Operation),
}

/// Computes the maximum value in a specified range of cells.
///
/// # Arguments
///
/// * `value1` - The starting cell (top-left corner) of the range.
/// * `value2` - The ending cell (bottom-right corner) of the range.
/// * `grid` - The 2D array representing the spreadsheet.
///
/// # Returns
///
/// * `Some(i32)` - The maximum value in the range if all cells are valid.
/// * `None` - If any cell in the range is invalid.
pub fn max_function(value1: Coordinates, value2: Coordinates, grid: &[Vec<Node>]) -> Option<i32> {
    let mut max_val = i32::MIN;
    for i in value1.row..=value2.row {
        for j in value1.col..=value2.col {
            if grid[i as usize][j as usize].valid {
                max_val = max(max_val, grid[i as usize][j as usize].node_value);
            } else {
                return None;
            }
        }
    }
    Some(max_val)
}

/// Computes the minimum value in a specified range of cells.
///
/// # Arguments
///
/// * `value1` - The starting cell (top-left corner) of the range.
/// * `value2` - The ending cell (bottom-right corner) of the range.
/// * `grid` - The 2D array representing the spreadsheet.
///
/// # Returns
///
/// * `Some(i32)` - The minimum value in the range if all cells are valid.
/// * `None` - If any cell in the range is invalid.
pub fn min_function(
    value1: Coordinates,
    value2: Coordinates,
    grid: &[Vec<Node>],
) -> Option<i32> {
    let mut min_val = i32::MAX;
    for i in value1.row..=value2.row {
        for j in value1.col..=value2.col {
            if grid[i as usize][j as usize].valid {
                min_val = min(min_val, grid[i as usize][j as usize].node_value);
            } else {
                return None;
            }
        }
    }
    Some(min_val)
}

/// Computes the average value in a specified range of cells.
///
/// # Arguments
///
/// * `value1` - The starting cell (top-left corner) of the range.
/// * `value2` - The ending cell (bottom-right corner) of the range.
/// * `grid` - The 2D array representing the spreadsheet.
///
/// # Returns
///
/// * `Some(i32)` - The average value in the range if all cells are valid.
/// * `None` - If any cell in the range is invalid.
pub fn avg_function(
    value1: Coordinates,
    value2: Coordinates,
    grid: &[Vec<Node>],
) -> Option<i32> {
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

/// Computes the sum of values in a specified range of cells.
///
/// # Arguments
///
/// * `value1` - The starting cell (top-left corner) of the range.
/// * `value2` - The ending cell (bottom-right corner) of the range.
/// * `grid` - The 2D array representing the spreadsheet.
///
/// # Returns
///
/// * `Some(i32)` - The sum of values in the range if all cells are valid.
/// * `None` - If any cell in the range is invalid.
pub fn sum_function(
    value1: Coordinates,
    value2: Coordinates,
    grid: &[Vec<Node>],
) -> Option<i32> {
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

/// Computes the standard deviation of values in a specified range of cells.
///
/// # Arguments
///
/// * `value1` - The starting cell (top-left corner) of the range.
/// * `value2` - The ending cell (bottom-right corner) of the range.
/// * `grid` - The 2D array representing the spreadsheet.
///
/// # Returns
///
/// * `Some(i32)` - The standard deviation of values in the range if all cells are valid.
/// * `None` - If any cell in the range is invalid.
pub fn stdev_function(
    value1: Coordinates,
    value2: Coordinates,
    grid: &[Vec<Node>],
) -> Option<i32> {
    let mut sum = 0f64;
    let mut count = 0;

    // First pass: calculate sum and count
    for i in value1.row..=value2.row {
        for j in value1.col..=value2.col {
            let node = &grid[i as usize][j as usize];
            if node.valid {
                sum += node.node_value as f64;
                count += 1;
            } else {
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

/// Checks if the given operation is an arithmetic operation.
///
/// # Arguments
///
/// * `op` - The operation to check.
///
/// # Returns
///
/// * `true` - If the operation is Add, Sub, Mul, or Div.
/// * `false` - Otherwise.
pub fn is_arithmetic(op: Operation) -> bool {
    matches!(op, Operation::Add | Operation::Sub | Operation::Mul | Operation::Div)
}
