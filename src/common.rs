/// Represents a value that can appear in a spreadsheet.
#[derive(Debug, Clone, PartialEq)]
//Clone required for `Vec<Value>`(in graph.rs) to implement `Clone`
pub enum Value {
    Cell(usize, usize),
    Const(isize),
    Oper(Option<Box<Value>>, Option<Box<Value>>, Operation), //value1 and value2, and the operation or command, respectively
    None,
}

impl Value {
    /// return row of Cell
    pub fn row(&self) -> usize {
        match self {
            Value::Cell(row, _) => *row,
            _ => panic!("Expected a Cell value"),
        }
    }
    /// return column of Cell
    pub fn col(&self) -> usize {
        match self {
            Value::Cell(_, col) => *col,
            _ => panic!("Expected a Cell value"),
        }
    }


    pub fn assign_row(&mut self, new_row: usize) {
        match self {
            Value::Cell(row, _) => {
                *row = new_row;
            }
            _ => panic!("Expected a Cell value"),
        }
    }


    pub fn assign_col(&mut self, new_col: usize) {
        match self {
            Value::Cell(_, col) => {
                *col = new_col;
            }
            _ => panic!("Expected a Cell value"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
//Needed as Value has implemented a clone
//Oper(Box<Value>, Box<Value>, Operation), //value1 and value2, and the operation or command, respectively
//   |                                  ^^^^^^^^^ the trait `Clone` is not implemented for `Operation`
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
    Web
}
