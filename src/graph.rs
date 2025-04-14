use std::collections::HashMap;

use crate::types::Coordinates;
use std::fmt;
#[derive(Debug)]
pub struct Node {
    pub node_value: i32,
    pub function: i32,
    pub value1: Coordinates,
    pub value2: Coordinates,
    pub position: Coordinates,
    // pub edges: HashMap<(usize, usize), ()>, // simulate HashTable: mapping Coordinates to empty value // debug // will change : 2hr
    pub valid: bool,
    pub dependents: Vec<Coordinates>, // stores the coordinates of dependent nodes
}

impl Node {
    pub fn get_value(&self) -> i32 {
        self.node_value
    }
    pub fn set_value(&mut self, value: i32) {
        self.node_value = value;
    }
    pub fn get_function(&self) -> i32 {
        self.function
    }
    pub fn set_function(&mut self, function: i32) {
        self.function = function;
    }
    pub fn set_position(&mut self, position: Coordinates) {
        self.position = position;
    }
    // getvalue1
    // getvalue2
    pub fn set_value1(&mut self, value1: Coordinates) {
        self.value1 = value1;
    }
    pub fn set_value2(&mut self, value2: Coordinates) {
        self.value2 = value2;
    }
    pub fn get_valid(&self) -> bool {
        self.valid
    }
    pub fn set_valid(&mut self, valid: bool) {
        self.valid = valid;
    }
    pub fn add_dep(&mut self, coord: Coordinates) {
        if !self.dependents.contains(&coord) {
            self.dependents.push(coord);
        }
    }
    pub fn remove_dep(&mut self, coord: Coordinates) {
        self.dependents.retain(|x| *x != coord);
    }
    pub fn get_dependents(&self) -> &Vec<Coordinates> {
        &self.dependents
    }
    pub fn set_dependents(&mut self, dependents: Vec<Coordinates>) {
        self.dependents = dependents;
    }
    pub fn check_cycle(&self, coord: Coordinates) -> bool {
        // check for cycle using efficient DFS
        // no need of visited and instack use dirty parent flags
        false
    }
    // topo sort -> no need -> already dependent lists present
}
/// remove graph will also improve memory usage

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Node {{ node_value: {}, function: {}, value1: ({}, {}), value2: ({}, {}), position: ({}, {}), valid: {}, dependents: {:?} }}",
            self.node_value,
            self.function,
            self.value1.row,
            self.value1.col,
            self.value2.row,
            self.value2.col,
            self.position.row,
            self.position.col,
            self.valid,
            self.dependents,
        )
    }
}
