// getting_things_updated function here
// all operations to be done here
use crate::functions::Operation;
use crate::functions::Value;
use crate::functions::avg_function;
use crate::functions::max_function;
use crate::functions::min_function;
use crate::functions::stdev_function;
use crate::functions::sum_function;
use crate::graph::Node;
use crate::types::Coordinates;

///// debug -> in add_and _break check which ops you are taking  ---> done

pub fn generate_grid(r: usize, c: usize) -> Vec<Vec<Node>> {
    (0..r)
        .map(|i| {
            (0..c)
                .map(|j| Node {
                    node_value: 0,
                    value1: Coordinates { row: -1, col: -1 },
                    value2: Coordinates { row: -1, col: -1 },
                    position: Coordinates {
                        row: i as i32,
                        col: j as i32,
                    },
                    op: Operation::Cons, // Default to constant assignment
                    valid: true,
                    visited: false,
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
    op: Operation,
    flag: bool, // debug
) {
    let target_row = target.row as usize;
    let target_col = target.col as usize;
    let target_cell = &mut graph[target_row][target_col];
    if flag {
        match op {
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
            }
            Operation::Add | Operation::Sub | Operation::Mul | Operation::Div => {
                // For binary operations, remove dependency from value1 and value2 - check cells and values
                if value1.col != -1 {
                    graph[value1.row as usize][value1.col as usize].add_dep(target);
                }
                if value2.col != -1 {
                    graph[value2.row as usize][value2.col as usize].add_dep(target);
                }
            }
            Operation::Cons | Operation::Slp => {
                // single cell operation just check value1 and apply the cases
                // cell
                if value1.col != -1 && value1.row != -1 {
                    graph[value1.row as usize][value1.col as usize].add_dep(target);
                }
                // value -- do nothing
            }
            _ => {} // will not reach here
        }
    } else {
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
            }
            Operation::Add | Operation::Sub | Operation::Mul | Operation::Div => {
                // For binary operations, remove dependency from value1 and value2 - check cells and values
                if old_value1.col != -1 {
                    graph[old_value1.row as usize][old_value1.col as usize].add_dep(target);
                }
                if old_value2.col != -1 {
                    graph[old_value2.row as usize][old_value2.col as usize].add_dep(target);
                }
            }
            Operation::Cons | Operation::Slp => {
                // single cell operation just check value1 and apply the cases
                // cell
                if old_value1.col != -1 && old_value1.row != -1 {
                    graph[old_value1.row as usize][old_value1.col as usize].add_dep(target);
                }
                // value -- do nothing
            }
            _ => {} // will not reach here
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
        match op {
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
            }
            Operation::Add | Operation::Sub | Operation::Mul | Operation::Div => {
                // For binary operations, remove dependency from value1 and value2 - check cells and values
                if value1.col != -1 {
                    graph[value1.row as usize][value1.col as usize].remove_dep(target);
                }
                if value2.col != -1 {
                    graph[value2.row as usize][value2.col as usize].remove_dep(target);
                }
            }
            Operation::Cons | Operation::Slp => {
                // single cell operation just check value1 and apply the cases
                // cell
                if value1.col != -1 && value1.row != -1 {
                    graph[value1.row as usize][value1.col as usize].remove_dep(target);
                }
                // value -- do nothing
            }
            _ => {} // will not reach here
        }
    } else {
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
            }
            Operation::Add | Operation::Sub | Operation::Mul | Operation::Div => {
                // For binary operations, remove dependency from value1 and value2 - check cells and values
                if old_value1.col != -1 {
                    graph[old_value1.row as usize][old_value1.col as usize].remove_dep(target);
                }
                if old_value2.col != -1 {
                    graph[old_value2.row as usize][old_value2.col as usize].remove_dep(target);
                }
            }
            Operation::Cons | Operation::Slp => {
                // single cell operation just check value1 and apply the cases
                // cell
                if old_value1.col != -1 && old_value1.row != -1 {
                    graph[old_value1.row as usize][old_value1.col as usize].remove_dep(target);
                }
                // value -- do nothing
            }
            _ => {} // will not reach here
        }
    }
}

// returns status to process_command
pub fn getting_things_updated(
    graph: &mut Vec<Vec<Node>>,
    target: Coordinates,
    value1: Coordinates,
    value2: Coordinates,
    op: Operation,
    // r: usize,
    // c: usize,
) -> i32 {
    // so at this point of time value1 and value2 will store the new ranges of the new function to be applied on target cell
    // will call update_node (breaking and adding edges)
    // break previous inward dependecies (will remove the target from its previous parents)
    // add new dependencies of target in parent's vector
    // old_op :
    break_edges(graph, value1, value2, target, op, false); // old dependencies
    add_edges(graph, value1, value2, target, op, true); // new dependencies
    // check if new dependecies introduces cycle
    if has_cycle(target, graph) {
        break_edges(graph, value1, value2, target, op, true); // new dependencies
        add_edges(graph, value1, value2, target, op, false); // old dependencies
        return 6;
    }

    // go to dependents of the cell and change their values
    graph[target.row as usize][target.col as usize].op = op;
    graph[target.row as usize][target.col as usize].value1.row = value1.row;
    graph[target.row as usize][target.col as usize].value1.col = value1.col;
    graph[target.row as usize][target.col as usize].value2.row = value2.row;
    graph[target.row as usize][target.col as usize].value2.col = value2.col;

    evaluate_node(graph, target);
    update_topo(graph, target);

    1 // success
}

pub fn update_topo(graph: &mut Vec<Vec<Node>>, start: Coordinates) {
    let mut stack = Vec::new();

    topological_sort(graph, start, &mut stack);

    while let Some(coord) = stack.pop() {
        evaluate_node(graph, coord);
    }

    // Reset visited flags so next call works correctly
    reset_visited(graph, start);
}

pub fn topological_sort(
    graph: &mut Vec<Vec<Node>>,
    node: Coordinates,
    stack: &mut Vec<Coordinates>,
) {
    let (i, j) = (node.row as usize, node.col as usize);
    if graph[i][j].visited {
        return;
    }

    // Mark visited before recursive calls
    graph[i][j].visited = true;

    // Clone dependents to avoid borrowing `graph[i][j]` during iteration
    let dependents = graph[i][j].dependents.clone();

    for dep in dependents {
        topological_sort(graph, dep, stack);
    }

    stack.push(node);
}

// function that sets node value according to its operation
pub fn evaluate_node(graph: &mut Vec<Vec<Node>>, coord: Coordinates) {
    let row = coord.row as usize;
    let col = coord.col as usize;

    // extract value1 and value2 before borrowing node
    let value1 = graph[row][col].value1;
    let value2 = graph[row][col].value2;

    let value1_valid = graph[value1.row as usize][value1.col as usize].valid;
    let value2_valid = graph[value2.row as usize][value2.col as usize].valid;
    let value1_node_value = graph[value1.row as usize][value1.col as usize].node_value;
    let value2_node_value = graph[value2.row as usize][value2.col as usize].node_value;
    let op = graph[row][col].op;

    // evaluate range result before mutable borrow
    let range_result = match op {
        Operation::Min => min_function(value1, value2, graph),
        Operation::Max => max_function(value1, value2, graph),
        Operation::Avg => avg_function(value1, value2, graph),
        Operation::Sum => sum_function(value1, value2, graph),
        Operation::Std => stdev_function(value1, value2, graph),
        _ => None,
    };

    let node = &mut graph[row][col];

    match node.op {
        Operation::Add => {
            // V V
            if node.value1.col == -1 && node.value2.col == -1 {
                node.valid = true;
                node.node_value = node.value1.row + node.value2.row;
            }
            // V C
            else if node.value1.col == -1 {
                if value2_valid {
                    node.valid = true;
                    node.node_value = node.value1.row + value2_node_value;
                } else {
                    node.valid = false;
                }
            }
            // C V
            else if node.value2.col == -1 {
                if value1_valid {
                    node.valid = true;
                    node.node_value = value1_node_value + node.value2.row;
                } else {
                    node.valid = false;
                }
            }
            // C C
            else {
                if value1_valid && value2_valid {
                    node.valid = true;
                    node.node_value = value1_node_value + value2_node_value;
                } else {
                    node.valid = false;
                }
            }
        }
        Operation::Sub => {
            // V V
            if node.value1.col == -1 && node.value2.col == -1 {
                node.valid = true;
                node.node_value = node.value1.row - node.value2.row;
            }
            // V C
            else if node.value1.col == -1 {
                if value2_valid {
                    node.valid = true;
                    node.node_value = node.value1.row - value2_node_value;
                } else {
                    node.valid = false;
                }
            }
            // C V
            else if node.value2.col == -1 {
                if value1_valid {
                    node.valid = true;
                    node.node_value = value1_node_value - node.value2.row;
                } else {
                    node.valid = false;
                }
            }
            // C C
            else {
                if value1_valid && value2_valid {
                    node.valid = true;
                    node.node_value = value1_node_value - value2_node_value;
                } else {
                    node.valid = false;
                }
            }
        }
        Operation::Mul => {
            // V V
            if node.value1.col == -1 && node.value2.col == -1 {
                node.valid = true;
                node.node_value = node.value1.row * node.value2.row;
            }
            // V C
            else if node.value1.col == -1 {
                if value2_valid {
                    node.valid = true;
                    node.node_value = node.value1.row * value2_node_value;
                } else {
                    node.valid = false;
                }
            }
            // C V
            else if node.value2.col == -1 {
                if value1_valid {
                    node.valid = true;
                    node.node_value = value1_node_value * node.value2.row;
                } else {
                    node.valid = false;
                }
            }
            // C C
            else {
                if value1_valid && value2_valid {
                    node.valid = true;
                    node.node_value = value1_node_value * value2_node_value;
                } else {
                    node.valid = false;
                }
            }
        }
        Operation::Div => {
            // handle ERR here /// debug
            // V V
            if node.value1.col == -1 && node.value2.col == -1 {
                if node.value2.row == 0 {
                    node.valid = false;
                } else {
                    node.valid = true;
                    node.node_value = node.value1.row / node.value2.row;
                }
            }
            // V C
            else if node.value1.col == -1 {
                if value2_valid {
                    if value2_node_value == 0 {
                        node.valid = false;
                    } else {
                        node.valid = true;
                        node.node_value = node.value1.row / value2_node_value;
                    }
                } else {
                    node.valid = false;
                }
            }
            // C V
            else if node.value2.col == -1 {
                if value1_valid {
                    if node.value2.row == 0 {
                        node.valid = false;
                    } else {
                        node.valid = true;
                        node.node_value = value1_node_value / node.value2.row;
                    }
                } else {
                    node.valid = false;
                }
            }
            // C C
            else {
                if value1_valid && value2_valid {
                    if value2_node_value == 0 {
                        node.valid = false;
                    } else {
                        node.valid = true;
                        node.node_value = value1_node_value / value2_node_value;
                    }
                } else {
                    node.valid = false;
                }
            }
        }
        // range based functions
        Operation::Min | Operation::Max | Operation::Avg | Operation::Sum | Operation::Std => {
            if let Some(result) = range_result {
                node.node_value = result;
                node.valid = true;
            } else {
                node.valid = false;
            }
        }
        // sleep function
        Operation::Slp => {
            // Handle sleep operation
            // Sleep for a specified duration
            // DEBUG - add this after testing
            // std::thread::sleep(std::time::Duration::from_secs(1));
        }
        _ => {}
    }
}

pub fn has_cycle(target: Coordinates, graph: &mut Vec<Vec<Node>>) -> bool {
    // check for cycle using iterative DFS
    // use stack
    // reset visited flags
    let mut stack = vec![target];
    graph[target.row as usize][target.col as usize].visited = true;
    while let Some(node1) = stack.pop() {
        // borrow checker error
        // let node = &mut graph[node1.row as usize][node1.col as usize];
        let dependents = graph[node1.row as usize][node1.col as usize]
            .dependents
            .clone();

        for dep in dependents {
            if dep.row == target.row && dep.col == target.col {
                // back edge to target → cycle
                reset_visited(graph, target);
                return true;
            }

            if !graph[dep.row as usize][dep.col as usize].visited {
                graph[dep.row as usize][dep.col as usize].visited = true;
                stack.push(dep);
            }
        }
    }

    reset_visited(graph, target);
    false
}

// redo dfs to reset flags
fn reset_visited(graph: &mut Vec<Vec<Node>>, start: Coordinates) {
    let mut stack = vec![start];
    graph[start.row as usize][start.col as usize].visited = false;

    while let Some(node1) = stack.pop() {
        // clone dependents first
        let dependents = graph[node1.row as usize][node1.col as usize]
            .dependents
            .clone();

        for dep in dependents {
            let dep_node = &mut graph[dep.row as usize][dep.col as usize];
            if dep_node.visited {
                dep_node.visited = false;
                stack.push(dep);
            }
        }
    }
}
