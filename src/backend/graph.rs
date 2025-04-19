//topo sort
//node
//adding
//breaking
//cycle
//reset
use crate::common::Value;
use crate::common::Operation;
use crate::backend::backend::Grid;
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
    pub fn get_node_value(&self) -> Option<isize> {
        if self.valid {
            Some(self.node_value)
        } else {
            None
        }
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
pub fn break_edges(grid:&mut Grid, target: Value, func: Value, flag: bool) {
    // break edges
    if flag {
        // break previous dependencies
        if let Some(Value::Cell(row, col)) = target {
            if let Some(Value::Oper(box1, box2, oper)) = func {
                match oper {
                    Operation::Sum | Operation::Avg | Operation::Max | Operation::Min => {
                        if let Some(Value::Cell(row1, col1)) = *box1 {
                            if let Some(Value::Cell(row2, col2)) = *box2 {
                                let node1 = grid.get_node(row1, col1);
                                let node2 = grid.get_node(row2, col2);
                                node1.remove_dep(target.clone());
                                node2.remove_dep(target.clone());
                            }
                        }
                    }
                    Operation::Add | Operation::Sub | Operation::Mul | Operation::Div => {
                        match *box1{
                            (Some(Value::Cell(row1, col1))) => {
                                grid.cells_vec[row1][col1].remove_dep(target.clone());
                            }
                            _ => {}
                        }
                        match *box2{
                            (Some(Value::Cell(row1, col1))) => {
                                grid.cells_vec[row1][col1].remove_dep(target.clone());
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

pub fn add_edges(grid:&mut Grid, target: Value, func: Value, flag: bool) {
    // add edges
}

/// Updates the edges of the graph based on target and function values.
/// flag is true when previous dependencies are to be broken and new dependecies are to be added
/// flag is false when only new dependencies are to be added and previous dependencies are to be broken (Circular dependency case)
pub fn update_edges(grid: &mut Grid, target: Value, func: Value, flag:bool) {
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
pub fn hasCycle(grid: &mut Grid, target: Value, func: Value) -> bool {
    
}

/// Returns the sequence of topological sort starting from target cell
pub fn get_sequence(grid: &mut Grid, target: Value, func: Value) -> Vec<Value> {

}