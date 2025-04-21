use crate::backend::backend::Grid;
use crate::common::*;

/// Calculates the maximum value from a range of cells
///
/// # Arguments
///
/// * `grid` - Mutable reference to the grid
/// * `row` - Row of the cell containing the MAX function
/// * `col` - Column of the cell containing the MAX function
///
/// # Returns
///
/// * `Some(isize)` - The maximum value found in the range
/// * `None` - If any cell in the range is invalid
pub fn max_function(grid: &mut Grid, row: usize, col: usize) -> Option<isize> {
    let mut max_val = isize::MIN;
    let node = grid.get_node(row, col);
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        if let (Value::Cell(row1, col1), Value::Cell(row2, col2)) = (*box1, *box2) {
            for i in row1..=row2 {
                for j in col1..=col2 {
                    let current_node = grid.get_node(i, j);
                    if !current_node.valid {
                        return None;
                    } else if current_node.node_value > max_val {
                        max_val = current_node.node_value;
                    }
                }
            }
        }
    }
    Some(max_val)
}

/// Calculates the minimum value from a range of cells
///
/// # Arguments
///
/// * `grid` - Mutable reference to the grid
/// * `row` - Row of the cell containing the MIN function
/// * `col` - Column of the cell containing the MIN function
///
/// # Returns
///
/// * `Some(isize)` - The minimum value found in the range
/// * `None` - If any cell in the range is invalid
pub fn min_function(grid: &mut Grid, row: usize, col: usize) -> Option<isize> {
    let mut min_val = isize::MAX;
    let node = grid.get_node(row, col);
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        if let (Value::Cell(row1, col1), Value::Cell(row2, col2)) = (*box1, *box2) {
            for i in row1..=row2 {
                for j in col1..=col2 {
                    let current_node = grid.get_node(i, j);
                    if !current_node.valid {
                        return None;
                    } else if current_node.node_value < min_val {
                        min_val = current_node.node_value;
                    }
                }
            }
        }
    }
    Some(min_val)
}

/// Calculates the sum of values from a range of cells
///
/// # Arguments
///
/// * `grid` - Mutable reference to the grid
/// * `row` - Row of the cell containing the SUM function
/// * `col` - Column of the cell containing the SUM function
///
/// # Returns
///
/// * `Some(isize)` - The sum of values in the range
/// * `None` - If any cell in the range is invalid
pub fn sum_function(grid: &mut Grid, row: usize, col: usize) -> Option<isize> {
    let mut sum_val = 0;
    let node = grid.get_node(row, col);
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        if let (Value::Cell(row1, col1), Value::Cell(row2, col2)) = (*box1, *box2) {
            for i in row1..=row2 {
                for j in col1..=col2 {
                    let current_node = grid.get_node(i, j);
                    if !current_node.valid {
                        return None;
                    } else {
                        sum_val += current_node.node_value;
                    }
                }
            }
        }
    }
    Some(sum_val)
}

/// Calculates the average value from a range of cells
///
/// # Arguments
///
/// * `grid` - Mutable reference to the grid
/// * `row` - Row of the cell containing the AVG function
/// * `col` - Column of the cell containing the AVG function
///
/// # Returns
///
/// * `Some(isize)` - The average value of the range (rounded down)
/// * `None` - If any cell in the range is invalid or if the range is empty
pub fn avg_function(grid: &mut Grid, row: usize, col: usize) -> Option<isize> {
    let mut sum_val = 0;
    let mut count = 0;
    let node = grid.get_node(row, col);
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        if let (Value::Cell(row1, col1), Value::Cell(row2, col2)) = (*box1, *box2) {
            for i in row1..=row2 {
                for j in col1..=col2 {
                    let current_node = grid.get_node(i, j);
                    if !current_node.valid {
                        return None;
                    }
                    sum_val += current_node.node_value;
                    count += 1;
                }
            }
        }
    }
    if count == 0 {
        None
    } else {
        Some(sum_val / count)
    }
}

/// Calculates the standard deviation of values from a range of cells
///
/// # Arguments
///
/// * `grid` - Mutable reference to the grid
/// * `row` - Row of the cell containing the STDEV function
/// * `col` - Column of the cell containing the STDEV function
///
/// # Returns
///
/// * `Some(isize)` - The standard deviation of values in the range (rounded to nearest integer)
/// * `None` - If any cell in the range is invalid
pub fn std_dev_function(grid: &mut Grid, row: usize, col: usize) -> Option<isize> {
    let node = grid.get_node(row, col);
    if let Some(Value::Oper(Some(box1), Some(box2), _)) = node.function.clone() {
        if let (Value::Cell(row1, col1), Value::Cell(row2, col2)) = (*box1, *box2) {
            let mut sum = 0f64;
            let mut count = 0;

            // First pass: sum and count valid nodes
            for i in row1..=row2 {
                for j in col1..=col2 {
                    let node_ref = grid.get_node(i, j);
                    if node_ref.valid {
                        sum += node_ref.node_value as f64;
                        count += 1;
                    } else {
                        return None;
                    }
                }
            }

            if count == 0 {
                return Some(0);
            }

            let mean = sum / count as f64;

            // Second pass: compute variance
            let mut variance = 0f64;
            for i in row1..=row2 {
                for j in col1..=col2 {
                    let node_ref = grid.get_node(i, j);
                    if !node_ref.valid {
                        return None;
                    }
                    let val = node_ref.node_value as f64;
                    variance += (val - mean) * (val - mean);
                }
            }

            let stdev = (variance / count as f64).sqrt().round() as isize;
            return Some(stdev);
        }
    }
    None
}

/// Adds two values which can be either constants or cell references
///
/// # Arguments
///
/// * `grid` - Mutable reference to the grid
/// * `row` - Row of the cell containing the ADD operation
/// * `col` - Column of the cell containing the ADD operation
///
/// # Returns
///
/// * `Some(isize)` - The sum of the two values
/// * `None` - If any referenced cell is invalid
pub fn add(grid: &mut Grid, row: usize, col: usize) -> Option<isize> {
    let node = grid.get_node(row, col);
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        let val1 = match *box1 {
            Value::Cell(row, col) => {
                let node = grid.get_node(row, col);
                if !node.valid {
                    return None;
                }
                node.node_value
            }
            Value::Const(c) => c,
            _ => return None,
        };

        let val2 = match *box2 {
            Value::Cell(row, col) => {
                let node = grid.get_node(row, col);
                if !node.valid {
                    return None;
                }
                node.node_value
            }
            Value::Const(c) => c,
            _ => return None,
        };

        Some(val1 + val2)
    } else {
        None
    }
}

/// Subtracts the second value from the first (both can be constants or cell references)
///
/// # Arguments
///
/// * `grid` - Mutable reference to the grid
/// * `row` - Row of the cell containing the SUB operation
/// * `col` - Column of the cell containing the SUB operation
///
/// # Returns
///
/// * `Some(isize)` - The result of the subtraction
/// * `None` - If any referenced cell is invalid
pub fn sub(grid: &mut Grid, row: usize, col: usize) -> Option<isize> {
    let node = grid.get_node(row, col);
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        let val1 = match *box1 {
            Value::Cell(row, col) => {
                let node = grid.get_node(row, col);
                if !node.valid {
                    return None;
                }
                node.node_value
            }
            Value::Const(c) => c,
            _ => return None,
        };

        let val2 = match *box2 {
            Value::Cell(row, col) => {
                let node = grid.get_node(row, col);
                if !node.valid {
                    return None;
                }
                node.node_value
            }
            Value::Const(c) => c,
            _ => return None,
        };

        Some(val1 - val2)
    } else {
        None
    }
}

/// Multiplies two values which can be either constants or cell references
///
/// # Arguments
///
/// * `grid` - Mutable reference to the grid
/// * `row` - Row of the cell containing the MUL operation
/// * `col` - Column of the cell containing the MUL operation
///
/// # Returns
///
/// * `Some(isize)` - The product of the two values
/// * `None` - If any referenced cell is invalid
pub fn mul(grid: &mut Grid, row: usize, col: usize) -> Option<isize> {
    let node = grid.get_node(row, col);
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        let val1 = match *box1 {
            Value::Cell(row, col) => {
                let node = grid.get_node(row, col);
                if !node.valid {
                    return None;
                }
                node.node_value
            }
            Value::Const(c) => c,
            _ => return None,
        };

        let val2 = match *box2 {
            Value::Cell(row, col) => {
                let node = grid.get_node(row, col);
                if !node.valid {
                    return None;
                }
                node.node_value
            }
            Value::Const(c) => c,
            _ => return None,
        };

        Some(val1 * val2)
    } else {
        None
    }
}

/// Divides the first value by the second (both can be constants or cell references)
///
/// # Arguments
///
/// * `grid` - Mutable reference to the grid
/// * `row` - Row of the cell containing the DIV operation
/// * `col` - Column of the cell containing the DIV operation
///
/// # Returns
///
/// * `Some(isize)` - The result of the division
/// * `None` - If any referenced cell is invalid or if division by zero is attempted
pub fn div(grid: &mut Grid, row: usize, col: usize) -> Option<isize> {
    let node = grid.get_node(row, col);
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        let val1 = match *box1 {
            Value::Cell(row, col) => {
                let node = grid.get_node(row, col);
                if !node.valid {
                    return None;
                }
                node.node_value
            }
            Value::Const(c) => c,
            _ => return None,
        };

        let val2 = match *box2 {
            Value::Cell(row, col) => {
                let node = grid.get_node(row, col);
                if !node.valid {
                    return None;
                }
                node.node_value
            }
            Value::Const(c) => c,
            _ => return None,
        };

        if val2 != 0 {
            Some(val1 / val2)
        } else {
            None // only this case possible
        }
    } else {
        None
    }
}

/// Sleep function that pauses execution for a specified number of seconds
///
/// # Arguments
///
/// * `grid` - Mutable reference to the grid
/// * `row` - Row of the cell containing the SLP function
/// * `col` - Column of the cell containing the SLP function
///
/// # Returns
///
/// * `Some(isize)` - The number of seconds slept
/// * `None` - If the referenced cell is invalid
pub fn slp(grid: &mut Grid, row: usize, col: usize) -> Option<isize> {
    let node = grid.get_node(row, col);
    if let Some(Value::Oper(Some(box1), Some(_box2), _oper)) = node.function.clone() {
        // check value1
        let val1 = match *box1 {
            Value::Cell(row, col) => {
                let node = grid.get_node(row, col);
                if !node.valid {
                    return None;
                }
                node.node_value
            }
            Value::Const(c) => c,
            _ => return None,
        };
        // sleep for that amount of time
        std::thread::sleep(std::time::Duration::from_secs(val1 as u64));
        // return value to be set to the cell
        Some(val1)
    } else {
        None
    }
}

/// Returns a constant value from either a constant or a cell reference
///
/// # Arguments
///
/// * `grid` - Mutable reference to the grid
/// * `row` - Row of the cell containing the CONS function
/// * `col` - Column of the cell containing the CONS function
///
/// # Returns
///
/// * `Some(isize)` - The referenced value
/// * `None` - If the referenced cell is invalid
pub fn cons(grid: &mut Grid, row: usize, col: usize) -> Option<isize> {
    // let sleep_time = 0;
    // sleep_time
    let node = grid.get_node(row, col);
    if let Some(Value::Oper(Some(box1), Some(_box2), _oper)) = node.function.clone() {
        // check value1
        let val1 = match *box1 {
            Value::Cell(row, col) => {
                let node = grid.get_node(row, col);
                if !node.valid {
                    return None;
                }
                node.node_value
            }
            Value::Const(c) => c,
            _ => return None,
        };
        // return value to be set to the cell
        Some(val1)
    } else {
        None
    }
}
// slp and cons -> left
