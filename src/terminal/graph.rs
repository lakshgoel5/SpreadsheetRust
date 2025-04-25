/// Graph module for dependency management in the spreadsheet.
///
/// This module provides data structures and functions to manage the dependency graph
/// between cells, detect cycles, and maintain relationships between cells that depend
/// on one another's values.
use crate::terminal::functions::Operation;
use crate::terminal::types::Coordinates;

/// Represents a node (cell) in the spreadsheet dependency graph.
///
/// Each node maintains its value, the operation it performs, references to its
/// operands, and a list of other cells that depend on it.
#[derive(Debug, Clone, Default)]
pub struct Node {
    /// List of cells that depend on this node's value
    pub dependents: Vec<Coordinates>,
    
    /// The computed value of this cell
    pub node_value: i32,
    
    /// First operand for the operation performed by this cell
    pub value1: Coordinates,
    
    /// Second operand for the operation performed by this cell
    pub value2: Coordinates,
    
    /// Position of this cell in the grid
    pub position: Coordinates,
    
    /// The operation performed by this cell
    pub op: Operation,
    
    /// Whether the cell contains a valid value
    pub valid: bool,
    
    /// Used during graph traversal algorithms (e.g., cycle detection)
    pub visited: bool,
}

impl Node {
    /// Gets the value of this node.
    ///
    /// # Returns
    ///
    /// The value of this node.
    pub fn get_value(&self) -> i32 {
        self.node_value
    }

    /// Sets the value of this node.
    ///
    /// # Arguments
    ///
    /// * `value` - The new value to set for this node.
    pub fn set_value(&mut self, value: i32) {
        self.node_value = value;
    }

    /// Sets the position of this node.
    ///
    /// # Arguments
    ///
    /// * `position` - The new position to set for this node.
    pub fn set_position(&mut self, position: Coordinates) {
        self.position = position;
    }

    /// Sets the first operand for the operation performed by this cell.
    ///
    /// # Arguments
    ///
    /// * `value1` - The coordinates of the first operand.
    pub fn set_value1(&mut self, value1: Coordinates) {
        self.value1 = value1;
    }

    /// Sets the second operand for the operation performed by this cell.
    ///
    /// # Arguments
    ///
    /// * `value2` - The coordinates of the second operand.
    pub fn set_value2(&mut self, value2: Coordinates) {
        self.value2 = value2;
    }

    /// Gets whether the cell contains a valid value.
    ///
    /// # Returns
    ///
    /// `true` if the cell contains a valid value, `false` otherwise.
    pub fn get_valid(&self) -> bool {
        self.valid
    }

    /// Sets whether the cell contains a valid value.
    ///
    /// # Arguments
    ///
    /// * `valid` - `true` if the cell contains a valid value, `false` otherwise.
    pub fn set_valid(&mut self, valid: bool) {
        self.valid = valid;
    }

    /// Adds a dependent cell to this node's dependents list if it's not already there.
    ///
    /// # Arguments
    ///
    /// * `cell` - The coordinates of the cell to add to dependents.
    pub fn add_dep(&mut self, cell: Coordinates) {
        if !self.dependents.iter().any(|x| x.row == cell.row && x.col == cell.col) {
            self.dependents.push(cell);
        }
    }

    /// Removes a dependent cell from this node's dependents list.
    ///
    /// # Arguments
    ///
    /// * `cell` - The coordinates of the cell to remove from dependents.
    pub fn remove_dep(&mut self, cell: Coordinates) {
        self.dependents.retain(|x| x.row != cell.row || x.col != cell.col);
    }

    /// Gets the list of cells that depend on this node's value.
    ///
    /// # Returns
    ///
    /// A reference to the list of dependent cells.
    pub fn get_dependents(&self) -> &Vec<Coordinates> {
        &self.dependents
    }

    /// Sets the list of cells that depend on this node's value.
    ///
    /// # Arguments
    ///
    /// * `dependents` - The new list of dependent cells.
    pub fn set_dependents(&mut self, dependents: Vec<Coordinates>) {
        self.dependents = dependents;
    }

}
