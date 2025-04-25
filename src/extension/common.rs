use serde::{Deserialize, Serialize};

/// Represents a value that can appear in a spreadsheet.
//Clone required for `Vec<Value>`(in graph.rs) to implement `Clone`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Cell(usize, usize),
    Const(isize),
    Oper(Option<Box<Value>>, Option<Box<Value>>, Operation), //value1 and value2, and the operation or command, respectively
}

impl Value {
    /// return row of Cell
    pub fn row(&self) -> usize {
        match self {
            Value::Cell(row, _) => *row,
            _ => 0,
        }
    }
    /// return column of Cell
    pub fn col(&self) -> usize {
        match self {
            Value::Cell(_, col) => *col,
            _ => 0,
        }
    }

    pub fn assign_row(&mut self, new_row: usize) {
        if let Value::Cell(row, _) = self {
            *row = new_row;
        }
    }

    pub fn assign_col(&mut self, new_col: usize) {
        if let Value::Cell(_, col) = self {
            *col = new_col;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
//Needed as Value has implemented a clone
//Oper(Box<Value>, Box<Value>, Operation), //value1 and value2, and the operation or command, respectively
//   |                                  ^^^^^^^^^ the trait `Clone` is not implemented for `Operation`
#[derive(Serialize, Deserialize)]
pub enum Operation {
    Cons,
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
    Avg,
    Sum,
    Std,
    Slp,
    EnableOutput,
    DisableOutput,
    ScrollTo,
    Left,
    Right,
    Up,
    Down,
    Quit,
    Web(String),
    Save(String),
    Undo,
    Redo,
    WebStart,
}
