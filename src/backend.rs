// getting_things_updated function here
// all operations to be done here
use crate::graph::Node;
use crate::types::Coordinates;
use crate::functions::Value;
use crate::functions::Operation;
use crate::functions::min_function;
use crate::functions::max_function;
use crate::functions::avg_function;
use crate::functions::sum_function;
use crate::functions::stdev_function;

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
    op : Operation,
    flag : bool, // debug
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


// returns status to process_command
pub fn getting_things_updated(
    graph: &mut Vec<Vec<Node>>,
    target: Coordinates,
    value1: Coordinates,
    value2: Coordinates,
    op: Operation,
    // r: usize,
    // c: usize,
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
    if has_cycle(target, graph) {
        break_edges(graph, value1, value2, target, op, true); // new dependencies
        add_edges(graph, value1, value2, target, op, false); // old dependencies
        return 6;
    }

    // go to dependents of the cell and change their values
    grid[target.row as usize][target.col as usize].op = op;
    grid[target.row as usize][target.col as usize].value1.row = value1.row;
    grid[target.row as usize][target.col as usize].value1.col = value1.col;
    grid[target.row as usize][target.col as usize].value2.row = value2.row;
    grid[target.row as usize][target.col as usize].value2.col = value2.col;

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
    let node_ref = &mut graph[i][j];

    if node_ref.visited {
        return;
    }

    node_ref.visited = true;

    for dep in &node_ref.dependents {
        topological_sort(graph, *dep, stack);
    }

    stack.push(node);
}

// function that sets node value according to its operation
pub fn evaluate_node (graph: &mut Vec<Vec<Node>>, coord: Coordinates) {
    let node = &mut graph[coord.row as usize][coord.col as usize];
    match node.op {
        Operation::Add => {
            // V V
            if node.value1.col == -1 && node.value2.col == -1 {
                node.valid = true;
                node.node_value = node.value1.row + node.value2.row;
            }
            // V C
            else if node.value1.col == -1 {
                if graph[node.value2.row as usize][node.value2.col as usize].valid {
                    node.valid = true;
                    node.node_value = node.value1.row + graph[node.value2.row as usize][node.value2.col as usize].node_value;
                } else {
                    node.valid = false;
                }
            }
            // C V
            else if node.value2.col == -1 {
                if graph[node.value1.row as usize][node.value1.col as usize].valid {
                    node.valid = true;
                    node.node_value = graph[node.value1.row as usize][node.value1.col as usize].node_value + node.value2.row;
                } else {
                    node.valid = false;
                }
            }
            // C C
            else {
                if graph[node.value1.row as usize][node.value1.col as usize].valid && graph[node.value2.row as usize][node.value2.col as usize].valid {
                    node.valid = true;
                    node.node_value = graph[node.value1.row as usize][node.value1.col as usize].node_value + graph[node.value2.row as usize][node.value2.col as usize].node_value;
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
                if graph[node.value2.row as usize][node.value2.col as usize].valid {
                    node.valid = true;
                    node.node_value = node.value1.row - graph[node.value2.row as usize][node.value2.col as usize].node_value;
                } else {
                    node.valid = false;
                }
            }
            // C V
            else if node.value2.col == -1 {
                if graph[node.value1.row as usize][node.value1.col as usize].valid {
                    node.valid = true;
                    node.node_value = graph[node.value1.row as usize][node.value1.col as usize].node_value - node.value2.row;
                } else {
                    node.valid = false;
                }
            }
            // C C
            else {
                if graph[node.value1.row as usize][node.value1.col as usize].valid && graph[node.value2.row as usize][node.value2.col as usize].valid {
                    node.valid = true;
                    node.node_value = graph[node.value1.row as usize][node.value1.col as usize].node_value - graph[node.value2.row as usize][node.value2.col as usize].node_value;
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
                if graph[node.value2.row as usize][node.value2.col as usize].valid {
                    node.valid = true;
                    node.node_value = node.value1.row * graph[node.value2.row as usize][node.value2.col as usize].node_value;
                } else {
                    node.valid = false;
                }
            }
            // C V
            else if node.value2.col == -1 {
                if graph[node.value1.row as usize][node.value1.col as usize].valid {
                    node.valid = true;
                    node.node_value = graph[node.value1.row as usize][node.value1.col as usize].node_value * node.value2.row;
                } else {
                    node.valid = false;
                }
            }
            // C C
            else {
                if graph[node.value1.row as usize][node.value1.col as usize].valid && graph[node.value2.row as usize][node.value2.col as usize].valid {
                    node.valid = true;
                    node.node_value = graph[node.value1.row as usize][node.value1.col as usize].node_value * graph[node.value2.row as usize][node.value2.col as usize].node_value;
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
                if graph[node.value2.row as usize][node.value2.col as usize].valid {
                    if graph[node.value2.row as usize][node.value2.col as usize].node_value == 0 {
                        node.valid = false;
                    } else {
                        node.valid = true;
                        node.node_value = node.value1.row / graph[node.value2.row as usize][node.value2.col as usize].node_value;
                    }
                } else {
                    node.valid = false;
                }
            }
            // C V
            else if node.value2.col == -1 {
                if graph[node.value1.row as usize][node.value1.col as usize].valid {
                    if node.value2.row == 0 {
                        node.valid = false;
                    } else {
                        node.valid = true;
                        node.node_value = graph[node.value1.row as usize][node.value1.col as usize].node_value / node.value2.row;
                    }
                } else {
                    node.valid = false;
                }
            }
            // C C
            else {
                if graph[node.value1.row as usize][node.value1.col as usize].valid && graph[node.value2.row as usize][node.value2.col as usize].valid {
                    if graph[node.value2.row as usize][node.value2.col as usize].node_value == 0 {
                        node.valid = false;
                    } else {
                        node.valid = true;
                        node.node_value = graph[node.value1.row as usize][node.value1.col as usize].node_value / graph[node.value2.row as usize][node.value2.col as usize].node_value;
                    }
                } else {
                    node.valid = false;
                }
            }
        }
        // range based functions
        Operation::Min => {
            if let Some(result) = min_function(node.value1, node.value2, graph) {
                node.node_value = result;
                node.valid = true;
            } else {
                node.valid = false;
            }
        }
        Operation::Max => {
            if let Some(result) = max_function(node.value1, node.value2, graph) {
                node.node_value = result;
                node.valid = true;
            } else {
                node.valid = false;
            }
        }
        Operation::Avg => {
            if let Some(result) = avg_function(node.value1, node.value2, graph) {
                node.node_value = result;
                node.valid = true;
            } else {
                node.valid = false;
            }
        }
        Operation::Sum => {
            if let Some(result) = sum_function(node.value1, node.value2, graph) {
                node.node_value = result;
                node.valid = true;
            } else {
                node.valid = false;
            }
        }
        Operation::Std => {
            if let Some(result) = stdev_function(node.value1, node.value2, graph) {
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
            /// DEBUG - add this after testing
            // std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}

pub fn has_cycle(target: Coordinates, graph: &mut Vec<Vec<Node>>) -> bool {
    // check for cycle using iterative DFS
    // use stack
    // reset visited flags
    let mut stack = vec![target];
    graph[target.row as usize][target.col as usize].visited = true;
    while let Some(node1) = stack.pop() {
        let node = &mut graph[node1.row as usize][node1.col as usize];

        for dep in &node.dependents {
            if dep.row == target.row && dep.col == target.col {
                // back edge to target → cycle
                reset_visited(graph, target);
                return true;
            }

            let dep_node = &mut graph[dep.row as usize][dep.col as usize];
            if !dep_node.visited {
                dep_node.visited = true;
                stack.push(*dep);
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
        let node = &mut graph[node1.row as usize][node1.col as usize];
        for dep in &node.dependents {
            let dep_node = &mut graph[dep.row as usize][dep.col as usize];
            if dep_node.visited {
                dep_node.visited = false;
                stack.push(*dep);
            }
        }
    }
}
