// getting_things_updated function here
// all operations to be done here
use crate::graph::Node;
use crate::types::Coordinates;
use crate::functions::Value;
use crate::functions::Operation;
// functions to be added :
// 1. generate_grid
// 2. add_edges
// 3. break_edges
// 4.

// static mut IS_DISABLED: bool = false;
// debug // change this
// static mut GRID: Option<Vec<Vec<types::Node>>> = None;

// fn generate_grid(r: usize, c: usize) {
//     unsafe {
//         GRID = Some(vec![vec![0; c + 1]; r + 1]);
//     }
// }

pub fn generate_grid(r: usize, c: usize) -> Vec<Vec<Node>> {
    (0..r)
        .map(|i| {
            (0..c)
                .map(|j| Node {
                    node_value: 0,
                    function: -1,
                    value1: Coordinates { row: -1, col: -1 },
                    value2: Coordinates { row: -1, col: -1 },
                    position: Coordinates {
                        row: i as i32,
                        col: j as i32,
                    },
                    op: Operation::Cons, // Default to constant assignment
                    valid: true,
                    dependents: Vec::new(),
                })
                .collect()
        })
        .collect()
}
// wrt new node -- inward

// flags : for these two functions
// true: when working with new dependencies : value1 and value2
// false: when working with old dependencies : old_value1 and old_value2
pub fn add_edges(
    graph: &mut Vec<Vec<Node>>,
    value1: Coordinates,
    value2: Coordinates,
    target: Coordinates,
    op : Operation,
    flag : bool, // debug
) {
    let target_row = target.row as usize;
    let target_col = target.col as usize;
    let target_cell = &mut graph[target_row][target_col];
    if flag {
        match target_cell.op {
            Operation::Sum | Operation::Avg | Operation::Max | Operation::Min => {
                // For range operations, remove from all cells in the range
                for i in value1.row..=value2.row {
                    for j in value1.col..=value2.col {
                        if i >= 0 && j >= 0 {
                            let r = i as usize;
                            let c = j as usize;
                            graph[r][c].add_dep(target);
                        }
                    }
                }
            },
            Operation::Add | Operation::Sub | Operation::Mul | Operation::Div =>
            {
                // For binary operations, remove dependency from value1 and value2 - check cells and values
                if value1.col != -1 {
                    graph[value1.row as usize][value1.col as usize].add_dep(target);
                }
                if value2.col != -1 {
                    graph[value2.row as usize][value2.col as usize].add_dep(target);
                }
            },
            Operation::Cons | Operation::Slp => {
                // single cell operation just check value1 and apply the cases
                // cell
                if value1.col != -1 && value1.row != -1 {
                    graph[value1.row as usize][value1.col as usize].add_dep(target);
                }
                // value -- do nothing
            },
            _ => {}  // will not reach here
        }
    }
    else {
        let old_value1 = target_cell.value1;
        let old_value2 = target_cell.value2;

        match target_cell.op {
            Operation::Sum | Operation::Avg | Operation::Max | Operation::Min => {
                // For range operations, remove from all cells in the range
                for i in old_value1.row..=old_value2.row {
                    for j in old_value1.col..=old_value2.col {
                        if i >= 0 && j >= 0 {
                            let r = i as usize;
                            let c = j as usize;
                            graph[r][c].add_dep(target);
                        }
                    }
                }
            },
            Operation::Add | Operation::Sub | Operation::Mul | Operation::Div =>
            {
                // For binary operations, remove dependency from value1 and value2 - check cells and values
                if old_value1.col != -1 {
                    graph[old_value1.row as usize][old_value1.col as usize].add_dep(target);
                }
                if old_value2.col != -1 {
                    graph[old_value2.row as usize][old_value2.col as usize].add_dep(target);
                }
            },
            Operation::Cons | Operation::Slp => {
                // single cell operation just check value1 and apply the cases
                // cell
                if old_value1.col != -1 && old_value1.row != -1 {
                    graph[old_value1.row as usize][old_value1.col as usize].add_dep(target);
                }
                // value -- do nothing
            },
            _ => {}  // will not reach here
        }
    }
}

// will fill when structure is more clear
// debug -- see the old dependecies where they are stored in the graph when fully made
// wrt new node -- inward


pub fn break_edges(
    graph: &mut Vec<Vec<Node>>,
    value1: Coordinates,
    value2: Coordinates,
    target: Coordinates,
    op: Operation,
    flag: bool, // debug
) {
    // Get the target cell
    let target_row = target.row as usize;
    let target_col = target.col as usize;
    let target_cell = &mut graph[target_row][target_col];
    if flag {
        match target_cell.op {
            Operation::Sum | Operation::Avg | Operation::Max | Operation::Min => {
                // For range operations, remove from all cells in the range
                for i in value1.row..=value2.row {
                    for j in value1.col..=value2.col {
                        if i >= 0 && j >= 0 {
                            let r = i as usize;
                            let c = j as usize;
                            graph[r][c].remove_dep(target);
                        }
                    }
                }
            },
            Operation::Add | Operation::Sub | Operation::Mul | Operation::Div =>
            {
                // For binary operations, remove dependency from value1 and value2 - check cells and values
                if value1.col != -1 {
                    graph[value1.row as usize][value1.col as usize].remove_dep(target);
                }
                if value2.col != -1 {
                    graph[value2.row as usize][value2.col as usize].remove_dep(target);
                }
            },
            Operation::Cons | Operation::Slp => {
                // single cell operation just check value1 and apply the cases
                // cell
                if value1.col != -1 && value1.row != -1 {
                    graph[value1.row as usize][value1.col as usize].remove_dep(target);
                }
                // value -- do nothing
            },
            _ => {}  // will not reach here
        }
    }
    else {
        // Get the previous dependencies (value1 and value2)
        let old_value1 = target_cell.value1;
        let old_value2 = target_cell.value2;
        
        // If old value1 is valid (not -1), remove dependencies
        // Check if the operation is range-based
        match target_cell.op {
            Operation::Sum | Operation::Avg | Operation::Max | Operation::Min => {
                // For range operations, remove from all cells in the range
                for i in old_value1.row..=old_value2.row {
                    for j in old_value1.col..=old_value2.col {
                        if i >= 0 && j >= 0 {
                            let r = i as usize;
                            let c = j as usize;
                            graph[r][c].remove_dep(target);
                        }
                    }
                }
            },
            Operation::Add | Operation::Sub | Operation::Mul | Operation::Div =>
            {
                // For binary operations, remove dependency from value1 and value2 - check cells and values
                if old_value1.col != -1 {
                    graph[old_value1.row as usize][old_value1.col as usize].remove_dep(target);
                }
                if old_value2.col != -1 {
                    graph[old_value2.row as usize][old_value2.col as usize].remove_dep(target);
                }
            },
            Operation::Cons | Operation::Slp => {
                // single cell operation just check value1 and apply the cases
                // cell
                if old_value1.col != -1 && old_value1.row != -1 {
                    graph[old_value1.row as usize][old_value1.col as usize].remove_dep(target);
                }
                // value -- do nothing
            },
            _ => {}  // will not reach here
        }
    }

}

pub fn has_cycle() -> bool {
    // check for cycle using efficient DFS
    // no need of visited and instack use dirty parent flags
    
    false
}
// returns status to process_command
pub fn getting_things_updated(
    graph: &mut Vec<Vec<Node>>,
    target: Coordinates,
    value1: Coordinates,
    value2: Coordinates,
    op: Operation,
    r: usize,
    c: usize,
    grid: &mut Vec<Vec<Node>>,
) -> i32 {
    // so at this point of time value1 and value2 will store the new ranges of the new function to be applied on target cell
    // will call update_node (breaking and adding edges)
    // break previous inward dependecies (will remove the target from its previous parents)
    // add new dependencies of target in parent's vector
    // old_op : 
    break_edges(graph, value1, value2, target, op, false);  // old dependencies
    add_edges(graph, value1, value2, target, op, true);  // new dependencies
    // check if new dependecies introduces cycle
    if has_cycle() {
        break_edges(graph, value1, value2, target, op, true); // new dependencies
        add_edges(graph, value1, value2, target, op, false); // old dependencies
        6
    }

    // go to dependents of the cell and change their values

    1 // success
}