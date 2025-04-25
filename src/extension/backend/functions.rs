use crate::extension::backend::backend::Grid;
use crate::extension::common::*;

/// Functions module for the spreadsheet extension backend.
/// This provides the core computational functions used in cell formulas.
/// Finds the maximum value in a range of cells.
///
/// # Arguments
///
/// * `grid` - A mutable reference to the spreadsheet grid containing all cells
/// * `row` - The row index of the cell containing this function
/// * `col` - The column index of the cell containing this function
///
/// # Returns
///
/// * `Some(isize)` - The maximum value found in the specified range if all cells are valid
/// * `None` - If any cell in the range is invalid or has an error
///
/// # Details
///
/// Iterates through all cells in the range specified by the cell's function parameters
/// (which should be Value::Cell coordinates for the top-left and bottom-right corners)
/// and returns the maximum value. If any cell in the range is invalid, returns None.
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

/// Finds the minimum value in a range of cells.
///
/// # Arguments
///
/// * `grid` - A mutable reference to the spreadsheet grid containing all cells
/// * `row` - The row index of the cell containing this function
/// * `col` - The column index of the cell containing this function
///
/// # Returns
///
/// * `Some(isize)` - The minimum value found in the specified range if all cells are valid
/// * `None` - If any cell in the range is invalid or has an error
///
/// # Details
///
/// Iterates through all cells in the range specified by the cell's function parameters
/// (which should be Value::Cell coordinates for the top-left and bottom-right corners)
/// and returns the minimum value. If any cell in the range is invalid, returns None.
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

/// Calculates the sum of all values in a range of cells.
///
/// # Arguments
///
/// * `grid` - A mutable reference to the spreadsheet grid containing all cells
/// * `row` - The row index of the cell containing this function
/// * `col` - The column index of the cell containing this function
///
/// # Returns
///
/// * `Some(isize)` - The sum of all values in the specified range if all cells are valid
/// * `None` - If any cell in the range is invalid or has an error
///
/// # Details
///
/// Iterates through all cells in the range specified by the cell's function parameters
/// (which should be Value::Cell coordinates for the top-left and bottom-right corners)
/// and adds their values. If any cell in the range is invalid, returns None.
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

/// Calculates the average (mean) of all values in a range of cells.
///
/// # Arguments
///
/// * `grid` - A mutable reference to the spreadsheet grid containing all cells
/// * `row` - The row index of the cell containing this function
/// * `col` - The column index of the cell containing this function
///
/// # Returns
///
/// * `Some(isize)` - The average of all values in the specified range if all cells are valid
/// * `None` - If any cell in the range is invalid or if the range is empty
///
/// # Details
///
/// Iterates through all cells in the range, adds their values, and divides by the count.
/// Returns None if any cell is invalid or if the range is empty (to prevent division by zero).
/// The result is truncated to an integer (as it returns isize).
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

/// Calculates the standard deviation of values in a range of cells.
///
/// # Arguments
///
/// * `grid` - A mutable reference to the spreadsheet grid containing all cells
/// * `row` - The row index of the cell containing this function
/// * `col` - The column index of the cell containing this function
///
/// # Returns
///
/// * `Some(isize)` - The standard deviation of values in the specified range if all cells are valid
/// * `None` - If any cell in the range is invalid or the calculation cannot be performed
/// * `Some(0)` - If the range is empty
///
/// # Details
///
/// Uses a two-pass algorithm:
/// 1. First pass: Calculate the mean of all values
/// 2. Second pass: Calculate the sum of squared differences from the mean
///
/// Then divides the sum of squared differences by the count to get the variance,
/// takes the square root, and rounds to the nearest integer.
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

/// Performs addition of two values.
///
/// # Arguments
///
/// * `grid` - A mutable reference to the spreadsheet grid containing all cells
/// * `row` - The row index of the cell containing this function
/// * `col` - The column index of the cell containing this function
///
/// # Returns
///
/// * `Some(isize)` - The result of adding the two values if both are valid
/// * `None` - If either value is invalid or an error occurs
///
/// # Details
///
/// Extracts the two operands from the cell's function (which can be cell references or constants),
/// verifies they are valid, and returns their sum.
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

/// Performs subtraction of two values.
///
/// # Arguments
///
/// * `grid` - A mutable reference to the spreadsheet grid containing all cells
/// * `row` - The row index of the cell containing this function
/// * `col` - The column index of the cell containing this function
///
/// # Returns
///
/// * `Some(isize)` - The result of subtracting the second value from the first if both are valid
/// * `None` - If either value is invalid or an error occurs
///
/// # Details
///
/// Extracts the two operands from the cell's function (which can be cell references or constants),
/// verifies they are valid, and returns the result of subtracting the second from the first.
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

/// Performs multiplication of two values.
///
/// # Arguments
///
/// * `grid` - A mutable reference to the spreadsheet grid containing all cells
/// * `row` - The row index of the cell containing this function
/// * `col` - The column index of the cell containing this function
///
/// # Returns
///
/// * `Some(isize)` - The result of multiplying the two values if both are valid
/// * `None` - If either value is invalid or an error occurs
///
/// # Details
///
/// Extracts the two operands from the cell's function (which can be cell references or constants),
/// verifies they are valid, and returns their product.
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

/// Performs division of two values.
///
/// # Arguments
///
/// * `grid` - A mutable reference to the spreadsheet grid containing all cells
/// * `row` - The row index of the cell containing this function
/// * `col` - The column index of the cell containing this function
///
/// # Returns
///
/// * `Some(isize)` - The result of dividing the first value by the second if both are valid
/// * `None` - If either value is invalid, the divisor is zero, or another error occurs
///
/// # Details
///
/// Extracts the two operands from the cell's function (which can be cell references or constants),
/// verifies they are valid, checks that the divisor is not zero, and returns the result of
/// dividing the first value by the second.
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

/// Performs a sleep operation, pausing execution for a specified number of seconds.
///
/// # Arguments
///
/// * `grid` - A mutable reference to the spreadsheet grid containing all cells
/// * `row` - The row index of the cell containing this function
/// * `col` - The column index of the cell containing this function
///
/// # Returns
///
/// * `Some(isize)` - The value that was slept for (in seconds) if valid
/// * `None` - If the sleep value is invalid or an error occurs
///
/// # Details
///
/// Extracts the sleep duration from the cell's function (which can be a cell reference or constant),
/// verifies it is valid, sleeps for that many seconds, and then returns the same value.
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

/// Sets a constant value in a cell.
///
/// # Arguments
///
/// * `grid` - A mutable reference to the spreadsheet grid containing all cells
/// * `row` - The row index of the cell containing this function
/// * `col` - The column index of the cell containing this function
///
/// # Returns
///
/// * `Some(isize)` - The constant value if valid
/// * `None` - If the value is invalid or an error occurs
///
/// # Details
///
/// Extracts the constant value from the cell's function (which can be a cell reference or constant),
/// verifies it is valid, and returns the value to be assigned to the cell.
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
