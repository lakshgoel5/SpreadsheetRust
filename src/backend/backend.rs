use crate::backend::graph::Node;
use crate::common::{Operation, Value};
use crate::parser::*;
/// Control Unit for data processing and updating values in Spreadsheeet.
/// The `Grid` struct is designed to store and manage a grid of `Cell` objects.

//init_backend(r,c) -> generate a grid of all nodes : returns void
//execute(value::cell, value::oper) -> update_graph(Node, value::oper), hasCycle(Box<>, value::cell), get_sequence(Box<>, value::cell), loop assign to Node = <functions>(Box<>, value::oper) -> return status
//process_command(r,c, string, value::Cell) -> parser, execute(value::cell, value::oper): return status
//get_value(value::cell): returns a cell_value

///Data structure to represent sheet
pub struct Grid {
    rows: usize,
    columns: usize,
    cells_vec: Vec<Vec<Node>>,
}

pub enum Status {
    Success,
    InvalidRange,
    UnrecognizedCmd,
    InvalidRowColumn,
    PrintEnabled,
    PrintDisabled,
    ScrollTo(usize, usize),
    Up,
    Down,
    Left,
    Right,
    Quit,
}

impl Grid {
    ///Function to initialize grid. Arguments are size of grid.
    pub fn new(rows: usize, columns: usize) -> Self {
        Grid {
            rows,
            columns,
            cells_vec: vec![vec![Node::new(0); columns]; rows],
        }
    }
    pub fn get_row_size(&self) -> usize {
        self.rows
    }
    pub fn get_column_size(&self) -> usize {
        self.columns
    }
    pub fn get_cell(&self, row: usize, column: usize) -> &Node {
        &self.cells_vec[row][column]
    }
    pub fn get_value(&self, row: usize, column: usize) -> i32 {
        self.cells_vec[row][column].get_node_value()
    }
}

///Struct that contains data structure as well as methods
pub struct Backend {
    grid: Grid,
}

impl Backend {
    ///Initializes Backend
    pub fn init_backend(rows: usize, columns: usize) -> Self {
        Backend {
            grid: Grid::new(rows, columns),
        }
    }
    ///Returns the value of cell
    pub fn get_value(&self, cell: Value) -> i32 {
        match cell {
            Value::Cell(row, col) => self.grid.get_value(row, col),
            _ => panic!("Expected a Cell value"),
        }
    }
    ///Checks for cycles and accordingly updates dependencies
    fn execute(cell: Value, operation: Value) -> Status {
        Status::Success
    }
    ///Takes command from frontend, calls the Parser, and sends the decoded command to execute function
    pub fn process_command(rows: usize, columns: usize, cmd: String) -> Status {
        match parser::validate(&cmd, &rows, &columns) {
            Some((None, Some(Value::Oper(None, None, op)))) => {
                return match op {
                    Operation::EnableOutput => Status::PrintEnabled,
                    Operation::DisableOutput => Status::PrintDisabled,
                    Operation::Left => Status::Left,
                    Operation::Right => Status::Right,
                    Operation::Up => Status::Up,
                    Operation::Down => Status::Down,
                    Operation::Quit => Status::Quit,
                    _ => Status::UnrecognizedCmd,
                };
            }
            Some((Some(Value::Cell(col, row)), Some(Value::Oper(None, None, op)))) => {
                return match op {
                    Operation::ScrollTo => Status::ScrollTo(row, col),
                    _ => Status::UnrecognizedCmd,
                };
            }
            Some((Some(Value::Cell(col, row)), Some(Value::Oper(box1, box2, op)))) => {
                return Self::execute(Value::Cell(col, row), Value::Oper(box1, box2, op));
            }
            _ => {
                return Status::UnrecognizedCmd;
            }
        }
    }
    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }
}
