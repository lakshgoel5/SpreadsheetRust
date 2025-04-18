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
