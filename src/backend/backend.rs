use crate::backend::functions::*;
use crate::backend::graph::Node;
use crate::backend::graph::get_sequence;
use crate::backend::graph::hasCycle;
use crate::backend::graph::update_edges;
use crate::common::{Operation, Value};
use crate::parser::*;
use serde::{Serialize, Deserialize};
/// Control Unit for data processing and updating values in Spreadsheeet.
/// The `Grid` struct is designed to store and manage a grid of `Cell` objects.

//init_backend(r,c) -> generate a grid of all nodes : returns void
//execute(value::cell, value::oper) -> update_edges(Node, value::oper), hasCycle(Box<>, value::cell), get_sequence(Box<>, value::cell), update_grid(sequence) -> return status
//update_grid(sequence) -> loop assign to Node = <functions>(Box<>, value::oper -> return bool
//process_command(r,c, string, value::Cell) -> parser, execute(value::cell, value::oper): return status
//get_value(value::cell): returns a cell_value

///Data structure to represent sheet
pub struct Grid {
    rows: isize,
    columns: isize,
    cells_vec: Vec<Vec<Node>>,
}
///Data structure to represent status of command
#[derive(PartialEq)]
pub enum Status {
    Success,
    InvalidRange,
    UnrecognizedCmd,
    InvalidRowColumn,
    CircularDependency,
    PrintEnabled,
    PrintDisabled,
    ScrollTo(isize, isize),
    Up,
    Down,
    Left,
    Right,
    Quit,
    Web,
}

impl Grid {
    ///Function to initialize grid. Arguments are size of grid.
    pub fn new(rows: isize, columns: isize) -> Self {
        Grid {
            rows,
            columns,
            cells_vec: vec![vec![Node::new(0); columns as usize]; rows as usize],
        }
    }
    pub fn get_row_size(&self) -> isize {
        self.rows
    }
    pub fn get_column_size(&self) -> isize {
        self.columns
    }
    pub fn get_node(&mut self, row: isize, column: isize) -> &mut Node {
        &mut self.cells_vec[row as usize][column as usize]
    }
    pub fn get_node_value(&self, row: isize, column: isize) -> Option<isize> {
        self.cells_vec[row as usize][column as usize].get_node_value()
    }
    // pub fn get_node_mut(&mut self, row: isize, column: isize) -> &mut Node {
    //     &mut self.cells_vec[row][column]
    // }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Valgrid {
    pub rows: isize,
    pub columns: isize,
    pub cells: Vec<Vec<isize>>,
}

///Struct that contains data structure as well as methods
pub struct Backend {
    grid: Grid,
}

impl Backend {
    ///Initializes Backend
    pub fn init_backend(rows: isize, columns: isize) -> Self {
        Backend {
            grid: Grid::new(rows + 1, columns + 1),
        }
    }
    ///Returns the value of cell
    pub fn get_node_value(&self, cell: Value) -> Option<isize> {
        match cell {
            Value::Cell(row, col) => self.grid.get_node_value(row , col),
            _ => panic!("Expected a Cell value"),
        }
    }
    pub fn get_valgrid(&self) -> Valgrid {
        Valgrid {
            rows: self.grid.get_row_size(),
            columns: self.grid.get_column_size(),
            cells: self
                .grid
                .cells_vec
                .iter()
                .map(|row| row.iter().map(|cell| cell.node_value).collect())
                .collect(),
        }
    }
    ///Iterates over the sequence of topological sort and updates values
    fn update_grid(&mut self, sequence: Vec<Value>) {
        for cell in sequence {
            if let Some(Value::Oper(_box1, _box2, oper)) =
                self.grid.get_node(cell.row(), cell.col()).function.clone()
            {
                match oper {
                    Operation::Sum => {
                        let sum = sum_function(&mut self.grid, cell.row(), cell.col());
                        match sum {
                            Some(val) => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].node_value = val;
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = false;
                            }
                        }
                    }
                    Operation::Min => {
                        let min = min_function(&mut self.grid, cell.row(), cell.col());
                        match min {
                            Some(val) => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].node_value = val;
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = false;
                            }
                        }
                    }
                    Operation::Max => {
                        let max = max_function(&mut self.grid, cell.row(), cell.col());
                        match max {
                            Some(val) => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].node_value = val;
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = false;
                            }
                        }
                    }
                    Operation::Avg => {
                        let avg = avg_function(&mut self.grid, cell.row(), cell.col());
                        match avg {
                            Some(val) => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].node_value = val;
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = false;
                            }
                        }
                    }
                    Operation::Std => {
                        let std_dev = std_dev_function(&mut self.grid, cell.row(), cell.col());
                        match std_dev {
                            Some(val) => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].node_value = val;
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = false;
                            }
                        }
                    }
                    Operation::Add => {
                        let ans = add(&mut self.grid, cell.row(), cell.col());
                        match ans {
                            Some(val) => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].node_value = val;
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = false;
                            }
                        }
                    }
                    Operation::Sub => {
                        let ans = sub(&mut self.grid, cell.row(), cell.col());
                        match ans {
                            Some(val) => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].node_value = val;
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = false;
                            }
                        }
                    }
                    Operation::Mul => {
                        let ans = mul(&mut self.grid, cell.row(), cell.col());
                        match ans {
                            Some(val) => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].node_value = val;
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = false;
                            }
                        }
                    }
                    Operation::Div => {
                        let ans = div(&mut self.grid, cell.row(), cell.col());
                        match ans {
                            Some(val) => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].node_value = val;
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = false;
                            }
                        }
                    }
                    Operation::Slp => {
                        let ans = slp(&mut self.grid, cell.row(), cell.col());
                        match ans {
                            Some(val) => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].node_value = val;
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = false;
                            }
                        }
                    }
                    Operation::Cons => {
                        let ans = cons(&mut self.grid, cell.row(), cell.col());
                        match ans {
                            Some(val) => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].node_value = val;
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row() as usize][cell.col() as usize].valid = false;
                            }
                        }
                    }
                    _ => {
                        // Handle other operations if needed
                    }
                }
            }
        }
    }
    ///Checks for cycles and accordingly updates dependencies
    fn execute(&mut self, cell: Value, func: Option<Value>) -> Status {
        //I want that if func has first and second box as value::const type, then just update graph and evaluate expression by sending Operation as well
        if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = func.clone() {
            if let (Value::Const(_val1), Value::Const(_val2)) = (*box1, *box2) {
                update_edges(&mut self.grid, cell.clone(), func.clone(), true); //debug check //add break edges
                // change cell's parameters here
                let node = self.grid.get_node(cell.row(), cell.col());
                node.function = func.clone();
                let sequence = get_sequence(&mut self.grid, cell.clone(), func.clone());
                self.update_grid(sequence.clone());
            } else {
                update_edges(&mut self.grid, cell.clone(), func.clone(), true);
                if hasCycle(&mut self.grid, cell.clone(), func.clone()) {
                    update_edges(&mut self.grid, cell.clone(), func.clone(), false);
                    return Status::CircularDependency;
                }
                // change cell's parameters here
                let node = self.grid.get_node(cell.row(), cell.col());
                node.function = func.clone();
                let sequence = get_sequence(&mut self.grid, cell.clone(), func.clone());
                self.update_grid(sequence.clone());
            }
        }
        Status::Success
    }
    ///Takes command from frontend, calls the Parser, and sends the decoded command to execute function
    pub fn process_command(&mut self, rows: isize, columns: isize, cmd: String) -> Status {
        match parser::validate(&cmd, &columns, &rows) {
            Some((None, Some(Value::Oper(None, None, op)))) => {
                return match op {
                    Operation::EnableOutput => Status::PrintEnabled,
                    Operation::DisableOutput => Status::PrintDisabled,
                    Operation::Left => Status::Left,
                    Operation::Right => Status::Right,
                    Operation::Up => Status::Up,
                    Operation::Down => Status::Down,
                    Operation::Quit => Status::Quit,
                    Operation::Web => Status::Web,
                    _ => Status::UnrecognizedCmd,
                };
            }
            Some((Some(Value::Cell(col, row)), Some(Value::Oper(None, None, op)))) => {
                return match op {
                    Operation::ScrollTo => Status::ScrollTo(col, row),
                    _ => Status::UnrecognizedCmd,
                };
            }
            Some((Some(Value::Cell(col, row)), Some(Value::Oper(box1, box2, op)))) => {
                // change here
                // either have to change parser or change the inside parts of box1 and box2
                return self.execute(Value::Cell(col, row), Some(Value::Oper(box1, box2, op)));
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