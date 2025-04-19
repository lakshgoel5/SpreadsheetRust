//topo sort
//node
//adding
//breaking
//cycle
//reset
use crate::backend::backend::Grid;
use crate::common::Operation;
use crate::common::Value;
///Data structure for strong data of each cell
/// Contains Dependency list, value, function and a few booleans
#[derive(Debug, Clone)]
//clone trait used due to vec![vec![Cell::new(0); columns]; rows]
pub struct Node {
    pub dependents: Vec<Value>,
    pub node_value: isize,
    pub function: Option<Value>,
    pub visited: bool,
    pub valid: bool,
}

impl Node {
    ///Initialises a Node
    pub fn new(val: isize) -> Self {
        Node {
            dependents: Vec::new(),
            node_value: val,
            function: None,
            visited: false,
            valid: true,
        }
    }
    pub fn get_node_value(&self) -> isize {
        self.node_value
    }
    pub fn remove_dep(&mut self, cell: Value) {
        self.dependents.retain(|x| x != &cell);
    }
    pub fn add_dep(&mut self, cell: Value) {
        if !self.dependents.contains(&cell) {
            self.dependents.push(cell);
        }
    }
    pub fn set_dependents(&mut self, dependents: Vec<Value>) {
        self.dependents = dependents;
    }
}

// update_edges
// hasCycle
// get_sequence

// flag -> true: break previous dependencies
/// Function to break edges concerned with target cell in the graph depending on flag
pub fn break_edges(grid: &mut Grid, target: Value, func: Option<Value>, flag: bool) {
    // break edges
    let old_func: Option<Value>;
    if flag {
        // break old dependencies (stored in grid)
        old_func = grid.get_node(target.row(), target.col()).function.clone();
    } else {
        old_func = func;
    }
    if let Some(Value::Oper(box1, box2, oper)) = old_func {
        match oper {
            Operation::Sum | Operation::Avg | Operation::Max | Operation::Min => {
                if let Some(Value::Cell(row1, col1)) = box1.as_deref() {
                    if let Some(Value::Cell(row2, col2)) = box2.as_deref() {
                        for i in *row1..=*row2 {
                            for j in *col1..=*col2 {
                                let node = grid.get_node(i, j);
                                node.remove_dep(target.clone());
                            }
                        }
                    }
                }
            }
            Operation::Add | Operation::Sub | Operation::Mul | Operation::Div => {
                if let Some(boxed_val) = box1 {
                    if let Value::Cell(row1, col1) = &*boxed_val {
                        let node1 = grid.get_node(*row1, *col1);
                        node1.remove_dep(target.clone());
                    }
                }
                if let Some(boxed_val) = box2 {
                    if let Value::Cell(row1, col1) = &*boxed_val {
                        let node1 = grid.get_node(*row1, *col1);
                        node1.remove_dep(target.clone());
                    }
                }
            }
            Operation::Cons | Operation::Slp => {
                // C
                if let Some(Value::Cell(row1, col1)) = box1.as_deref() {
                    let node1 = grid.get_node(*row1, *col1);
                    node1.remove_dep(target.clone());
                }
                // V -> do nothing
            }
            _ => {}
        }
    }
}

// flag -> true: add new dependencies
/// Function to add edges concerned with target cell in the graph depending on flag
pub fn add_edges(grid: &mut Grid, target: Value, func: Option<Value>, flag: bool) {
    // add edges
    let old_func: Option<Value>;
    if flag {
        // add new dependencies
        old_func = func;
    } else {
        old_func = grid.get_node(target.row(), target.col()).function.clone();
    }
    if let Some(Value::Oper(box1, box2, oper)) = old_func {
        match oper {
            Operation::Sum | Operation::Avg | Operation::Max | Operation::Min => {
                if let Some(Value::Cell(row1, col1)) = box1.as_deref() {
                    if let Some(Value::Cell(row2, col2)) = box2.as_deref() {
                        for i in *row1..=*row2 {
                            for j in *col1..=*col2 {
                                let node = grid.get_node(i, j);
                                node.add_dep(target.clone());
                            }
                        }
                    }
                }
            }
            Operation::Add | Operation::Sub | Operation::Mul | Operation::Div => {
                if let Some(boxed_val) = box1 {
                    if let Value::Cell(row1, col1) = &*boxed_val {
                        let node1 = grid.get_node(*row1, *col1);
                        node1.add_dep(target.clone());
                    }
                }
                if let Some(boxed_val) = box2 {
                    if let Value::Cell(row1, col1) = &*boxed_val {
                        let node1 = grid.get_node(*row1, *col1);
                        node1.add_dep(target.clone());
                    }
                }
            }
            Operation::Cons | Operation::Slp => {
                // C
                if let Some(Value::Cell(row1, col1)) = box1.as_deref() {
                    let node1 = grid.get_node(*row1, *col1);
                    node1.add_dep(target.clone());
                }
                // V -> do nothing
            }
            _ => {}
        }
    }
}

/// Updates the edges of the graph based on target and function values.
/// flag is true when previous dependencies are to be broken and new dependecies are to be added
/// flag is false when only new dependencies are to be added and previous dependencies are to be broken (Circular dependency case)
pub fn update_edges(grid: &mut Grid, target: Value, func: Option<Value>, flag: bool) {
    // so here in update edges -> func will contain the 3 value tuple (new)
    // target will always be a cell
    if let Value::Cell(row, col) = target {
        if let Some(Value::Oper(ref box1, ref box2, ref _oper)) = func {
            // passing target row col to access the node in functions
            break_edges(grid, target.clone(), func.clone(), flag);
            add_edges(grid, target.clone(), func.clone(), flag);
        }
    }
}

/// Checks for circular dependency in graph using DFS
pub fn hasCycle(grid: &mut Grid, target: Value, func: Option<Value>) -> bool {
    let mut stack = vec![target.clone()];
    let node = grid.get_node(target.row(), target.col());
    node.visited = true;
    while let Some(Value::Cell(row, col)) = stack.pop() {
        let dependents = grid.get_node(row, col).dependents.clone();
        for dep in dependents {
            if let Value::Cell(dep_r, dep_c) = dep {
                let dep_node = grid.get_node(dep_r, dep_c);
                if dep_node.visited {
                    // cycle detected
                    reset_visited(grid, target.clone());
                    return true;
                } else {
                    dep_node.visited = true;
                    stack.push(dep);
                }
            }
        }
    }
    reset_visited(grid, target.clone());
    false
}

/// This function does dfs in order to efficiently reset visited flags to false
pub fn reset_visited(grid: &mut Grid, start: Value) {
    if let Value::Cell(row, col) = start {
        let mut stack = vec![start];
        grid.get_node(row, col).visited = false;

        while let Some(Value::Cell(r, c)) = stack.pop() {
            let dependents = grid.get_node(r, c).dependents.clone();

            for dep in dependents {
                if let Value::Cell(dep_r, dep_c) = dep {
                    let dep_node = grid.get_node(dep_r, dep_c);
                    if dep_node.visited {
                        dep_node.visited = false;
                        stack.push(dep);
                    }
                }
            }
        }
    }
}

/// Returns the sequence of topological sort starting from target cell
pub fn get_sequence(grid: &mut Grid, target: Value, func: Option<Value>) -> Vec<Value> {
    let mut stack = Vec::new();
    topological_sort(grid, target, &mut stack);
    return stack;
}

pub fn topological_sort(grid: &mut Grid, target: Value, stack: &mut Vec<Value>) {
    if let Value::Cell(row, col) = target {
        let node = grid.get_node(row, col);
        if node.visited {
            return;
        }
        node.visited = true;

        for dep in node.dependents.clone() {
            topological_sort(grid, dep, stack);
            // if let Value::Cell(dep_row, dep_col) = dep {
            //     topological_sort(grid, Value::Cell(dep_row, dep_col), stack);
            // }
        }
        stack.push(target);
    }
}
