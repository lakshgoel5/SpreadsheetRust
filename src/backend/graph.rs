//topo sort
//node
//adding
//breaking
//cycle
//reset
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
            valid: false,
        }
    }
    pub fn get_node_value(&self) -> isize {
        self.node_value
    }
}

// update_edges
// hasCycle
// get_sequence
pub fn break_edges(grid:&Grid, target: Value, func: Value, flag: bool) {
    // break edges

}

pub fn add_edges(grid:&Grid, target: Value, func: Value, flag: bool) {
    // add edges
}

/// Updates the edges of the graph based on target and function values.
/// flag is true when previous dependencies are to be broken and new dependecies are to be added
/// flag is false when only new dependencies are to be added and previous dependencies are to be broken (Circular dependency case)
pub fn update_edges(grid: &Grid, target: Value, func: Value, flag:bool) {
    // target will always be a cell
    if let Some(Value::Cell(row, col)) = target {
        if let Some(Value::Oper(box1, box2, _oper)) = func {
            // passing target row col to access the node in functions
            break_edges(grid, target.clone(), func.clone(), flag);
            add_edges(grid, target.clone(), func.clone(), flag);
        }
    }
}

/// Checks for circular dependency in graph using DFS
pub fn hasCycle(grid: &Grid, target: Value, func: Value) -> bool {

}

/// Returns the sequence of topological sort starting from target cell
pub fn get_sequence(grid: &Grid, target: Value, func: Value) -> Vec<Value> {

}