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
pub struct Cell {
    pub dependents: Vec<Value>,
    pub node_value: i32,
    pub function: Option<Value>,
    pub visited: bool,
    pub valid: bool,
}

impl Cell {
    ///Initialises a cell
    pub fn new(val: i32) -> Self {
        Cell {
            dependents: Vec::new(),
            node_value: val,
            function: None,
            visited: false,
            valid: false,
        }
    }
    pub fn get_cell_value(&self) -> i32 {
        self.node_value
    }
}
