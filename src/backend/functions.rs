use crate::backend::backend::Grid;
use crate::backend::graph::Node;
use crate::common::*;

//would give you reference of grid reference and a node reference
//write all functions given in common Operations
// debug -> change this to Option for ERR cases
pub fn max_function(grid: &Grid, node: &Node) -> Option<isize> {
    let mut max_val = isize::MIN;
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        if let (Value::Cell(row1, col1), Value::Cell(row2, col2)) = (*box1, *box2) {
            for i in row1..=row2 {
                for j in col1..=col2 {
                    let current_node = grid.get_node(i, j);
                    if !current_node.valid {
                        return None;
                    }
                    else {
                        if current_node.node_value > max_val {
                            max_val = current_node.node_value;
                        }
                    }
                }
            }
        }
    }
    Some(max_val)
}

pub fn min_function(grid: &Grid, node: &Node) -> Option<isize> {
    let mut min_val = isize::MAX;
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        if let (Value::Cell(row1, col1), Value::Cell(row2, col2)) = (*box1, *box2) {
            for i in row1..=row2 {
                for j in col1..=col2 {
                    let current_node = grid.get_node(i, j);
                    if !current_node.valid {
                        return None;
                    }
                    else {
                        if current_node.node_value < min_val {
                            min_val = current_node.node_value;
                        }
                    }
                }
            }
        }
    }
    Some(min_val)
}

pub fn sum_function(grid: &Grid, node: &Node) -> Option<isize> {
    let mut sum_val = 0;
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        if let (Value::Cell(row1, col1), Value::Cell(row2, col2)) = (*box1, *box2) {
            for i in row1..=row2 {
                for j in col1..=col2 {
                    let current_node = grid.get_node(i, j);
                    if !current_node.valid {
                        return None;
                    }
                    else {
                        sum_val += current_node.node_value;
                    }
                }
            }
        }
    }
    Some(sum_val)
}

pub fn avg_function(grid: &Grid, node: &Node) -> Option<isize> {
    let mut sum_val = 0;
    let mut count = 0;
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        if let (Value::Cell(row1, col1), Value::Cell(row2, col2)) = (*box1, *box2) {
            for i in row1..=row2 {
                for j in col1..=col2 {
                    let current_node = grid.get_node(i, j);
                    if !current_node.valid {
                        return None;
                    }
                    sum_val += grid.get_node_value(i, j);
                    count += 1;
                }
            }
        }
    }
    if count == 0 { None } else { Some(sum_val / count) }
}

pub fn std_dev_function(grid: &Grid, node: &Node) -> Option<isize> {
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


pub fn add(grid: &Grid, node: &Node) -> Option<isize> {
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

pub fn sub(grid: &Grid, node: &Node) -> Option<isize> {
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

pub fn mul(grid: &Grid, node: &Node) -> Option<isize> {
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

pub fn div(grid: &Grid, node: &Node) -> Option<isize> {
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
            None  // only this case possible
        }
    } else {
        None
    }
}

// pub fn sleep_function(cells: &Vec<Vec<Node>>, sleep_value: Value) -> Option<isize> {
//     let sleep_time = 0;
//     sleep_time
// }

// slp and cons -> left