use crate::backend::backend::Grid;
use crate::backend::graph::Node;
use crate::common::*;

//would give you reference of grid reference and a node reference
//write all functions given in common Operations
pub fn max(grid: &Grid, node: &Node) -> isize {
    let mut max_val = 0;
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        if let (Value::Cell(row1, col1), Value::Cell(row2, col2)) = (*box1, *box2) {
            for i in row1..=row2 {
                for j in col1..=col2 {
                    let val = grid.get_node_value(i, j);
                    if val > max_val {
                        max_val = val;
                    }
                }
            }
        }
    }
    max_val
}

pub fn min(grid: &Grid, node: &Node) -> isize {
    let mut min_val = isize::MAX;
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        if let (Value::Cell(row1, col1), Value::Cell(row2, col2)) = (*box1, *box2) {
            for i in row1..=row2 {
                for j in col1..=col2 {
                    let val = grid.get_node_value(i, j);
                    if val < min_val {
                        min_val = val;
                    }
                }
            }
        }
    }
    min_val
}

pub fn sum(grid: &Grid, node: &Node) -> isize {
    let mut sum_val = 0;
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        if let (Value::Cell(row1, col1), Value::Cell(row2, col2)) = (*box1, *box2) {
            for i in row1..=row2 {
                for j in col1..=col2 {
                    sum_val += grid.get_node_value(i, j);
                }
            }
        }
    }
    sum_val
}

pub fn avg(grid: &Grid, node: &Node) -> f32 {
    let mut sum_val = 0;
    let mut count = 0;
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        if let (Value::Cell(row1, col1), Value::Cell(row2, col2)) = (*box1, *box2) {
            for i in row1..=row2 {
                for j in col1..=col2 {
                    sum_val += grid.get_node_value(i, j);
                    count += 1;
                }
            }
        }
    }
    if count > 0 {
        sum_val as f32 / count as f32
    } else {
        0.0
    }
}

pub fn std_dev(grid: &Grid, node: &Node) -> f64 {
    if let Some(Value::Oper(Some(box1), Some(box2), _)) = node.function.clone() {
        match (*box1, *box2) {
            (Value::Cell(row1, col1), Value::Cell(row2, col2)) => {
                let mut values = vec![];

                for i in row1..=row2 {
                    for j in col1..=col2 {
                        values.push(grid.get_node_value(i, j) as f64);
                    }
                }

                let n = values.len();
                if n <= 1 {
                    return 0.0; // Std dev is zero or undefined for 0 or 1 element
                }

                let mean: f64 = values.iter().sum::<f64>() / n as f64;
                let variance: f64 =
                    values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1) as f64;

                variance.sqrt()
            }
            _ => 0.0,
        }
    } else {
        0.0
    }
}

pub fn add(grid: &Grid, node: &Node) -> isize {
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        let val1 = match *box1 {
            Value::Cell(row, col) => grid.get_node_value(row, col),
            Value::Const(c) => c,
            _ => 0,
        };

        let val2 = match *box2 {
            Value::Cell(row, col) => grid.get_node_value(row, col),
            Value::Const(c) => c,
            _ => 0,
        };

        val1 + val2
    } else {
        0
    }
}

pub fn sub(grid: &Grid, node: &Node) -> isize {
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        let val1 = match *box1 {
            Value::Cell(row, col) => grid.get_node_value(row, col),
            Value::Const(c) => c,
            _ => 0,
        };

        let val2 = match *box2 {
            Value::Cell(row, col) => grid.get_node_value(row, col),
            Value::Const(c) => c,
            _ => 0,
        };

        val1 - val2
    } else {
        0
    }
}

pub fn mul(grid: &Grid, node: &Node) -> isize {
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        let val1 = match *box1 {
            Value::Cell(row, col) => grid.get_node_value(row, col),
            Value::Const(c) => c,
            _ => 0,
        };

        let val2 = match *box2 {
            Value::Cell(row, col) => grid.get_node_value(row, col),
            Value::Const(c) => c,
            _ => 0,
        };

        val1 * val2
    } else {
        0
    }
}

pub fn div(grid: &Grid, node: &Node) -> isize {
    if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = node.function.clone() {
        let val1 = match *box1 {
            Value::Cell(row, col) => grid.get_node_value(row, col),
            Value::Const(c) => c,
            _ => 0,
        };

        let val2 = match *box2 {
            Value::Cell(row, col) => grid.get_node_value(row, col),
            Value::Const(c) => c,
            _ => 0,
        };

        if val2 != 0 {
            val1 / val2
        } else {
            0 // Handle division by zero
        }
    } else {
        0
    }
}

pub fn sleep_function(cells: &Vec<Vec<Node>>, sleep_value: Value) -> isize {
    let sleep_time = 0;
    sleep_time
}
