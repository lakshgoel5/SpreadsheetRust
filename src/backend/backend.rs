use crate::backend::functions::*;
use crate::backend::graph::Node;
use crate::common::{Operation, Value};
use crate::parser::*;
use crate::backend::graph::update_edges;
/// Control Unit for data processing and updating values in Spreadsheeet.
/// The `Grid` struct is designed to store and manage a grid of `Cell` objects.

//init_backend(r,c) -> generate a grid of all nodes : returns void
//execute(value::cell, value::oper) -> update_edges(Node, value::oper), hasCycle(Box<>, value::cell), get_sequence(Box<>, value::cell), update_grid(sequence) -> return status
//update_grid(sequence) -> loop assign to Node = <functions>(Box<>, value::oper -> return bool
//process_command(r,c, string, value::Cell) -> parser, execute(value::cell, value::oper): return status
//get_value(value::cell): returns a cell_value

///Data structure to represent sheet
pub struct Grid {
    rows: usize,
    columns: usize,
    cells_vec: Vec<Vec<Node>>,
}
///Data structure to represent status of command
pub enum Status {
    Success,
    InvalidRange,
    UnrecognizedCmd,
    InvalidRowColumn,
    CircularDependency,
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
    pub fn get_node(&self, row: usize, column: usize) -> &mut Node {
        &mut self.cells_vec[row][column]
    }
    pub fn get_node_value(&self, row: usize, column: usize) -> isize {
        self.cells_vec[row][column].get_node_value()
    }
    // pub fn get_node_mut(&mut self, row: usize, column: usize) -> &mut Node {
    //     &mut self.cells_vec[row][column]
    // }
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
    pub fn get_node_value(&self, cell: Value) -> isize {
        match cell {
            Value::Cell(row, col) => self.grid.get_node_value(row, col),
            _ => panic!("Expected a Cell value"),
        }
    }
    ///Iterates over the sequence of topological sort and updates values
    fn update_grid(&mut self, sequence: Vec<Value>) {
        for cell in sequence {
            if let Some(Value::Oper(box1, box2, oper)) =
                self.grid.get_node(cell.row(), cell.col()).function.clone()
            {
                match oper {
                    Operation::Sum => {
                        let sum = sum_function(&self.grid, &self.grid.get_node(cell.row(), cell.col()));
                        match sum {
                            Some(val) => {
                                self.grid.cells_vec[cell.row()][cell.col()].node_value = val;
                                self.grid.cells_vec[cell.row()][cell.col()].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row()][cell.col()].valid = false;
                            }
                        }
                    }
                    Operation::Min => {
                        let min = min_function(&self.grid, &self.grid.get_node(cell.row(), cell.col()));
                        match min {
                            Some(val) => {
                                self.grid.cells_vec[cell.row()][cell.col()].node_value = val;
                                self.grid.cells_vec[cell.row()][cell.col()].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row()][cell.col()].valid = false;
                            }
                        }
                    }
                    Operation::Max => {
                        let max = max_function(&self.grid, &self.grid.get_node(cell.row(), cell.col()));
                        match max {
                            Some(val) => {
                                self.grid.cells_vec[cell.row()][cell.col()].node_value = val;
                                self.grid.cells_vec[cell.row()][cell.col()].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row()][cell.col()].valid = false;
                            }
                        }
                    }
                    Operation::Avg => {
                        let avg = avg_function(&self.grid, &self.grid.get_node(cell.row(), cell.col()));
                        match avg {
                            Some(val) => {
                                self.grid.cells_vec[cell.row()][cell.col()].node_value = val;
                                self.grid.cells_vec[cell.row()][cell.col()].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row()][cell.col()].valid = false;
                            }
                        }
                    }
                    Operation::Std => {
                        let std_dev = std_dev_function(&self.grid, &self.grid.get_node(cell.row(), cell.col()));
                        match std_dev {
                            Some(val) => {
                                self.grid.cells_vec[cell.row()][cell.col()].node_value = val;
                                self.grid.cells_vec[cell.row()][cell.col()].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row()][cell.col()].valid = false;
                            }
                        }
                    }
                    Operation::Add => {
                        let ans = add(&self.grid, &self.grid.get_node(cell.row(), cell.col()));
                        match ans {
                            Some(val) => {
                                self.grid.cells_vec[cell.row()][cell.col()].node_value = val;
                                self.grid.cells_vec[cell.row()][cell.col()].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row()][cell.col()].valid = false;
                            }
                        }
                        
                    }
                    Operation::Sub => {
                        let ans = sub(&self.grid, &self.grid.get_node(cell.row(), cell.col()));
                        match ans {
                            Some(val) => {
                                self.grid.cells_vec[cell.row()][cell.col()].node_value = val;
                                self.grid.cells_vec[cell.row()][cell.col()].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row()][cell.col()].valid = false;
                            }
                        }
                    }
                    Operation::Mul => {
                        let ans = mul(&self.grid, &self.grid.get_node(cell.row(), cell.col()));
                        match ans {
                            Some(val) => {
                                self.grid.cells_vec[cell.row()][cell.col()].node_value = val;
                                self.grid.cells_vec[cell.row()][cell.col()].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row()][cell.col()].valid = false;
                            }
                        }
                    }
                    Operation::Div => {
                        let ans = div(&self.grid, &self.grid.get_node(cell.row(), cell.col()));
                        match ans {
                            Some(val) => {
                                self.grid.cells_vec[cell.row()][cell.col()].node_value = val;
                                self.grid.cells_vec[cell.row()][cell.col()].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row()][cell.col()].valid = false;
                            }
                        }
                    }
                    Operation::Slp => {
                        let ans = slp(&self.grid, &self.grid.get_node(cell.row(), cell.col()));
                        match ans {
                            Some(val) => {
                                self.grid.cells_vec[cell.row()][cell.col()].node_value = val;
                                self.grid.cells_vec[cell.row()][cell.col()].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row()][cell.col()].valid = false;
                            }
                        }
                    }
                    Operation::Cons => {
                        let ans = cons(&self.grid, &self.grid.get_node(cell.row(), cell.col()));
                        match ans {
                            Some(val) => {
                                self.grid.cells_vec[cell.row()][cell.col()].node_value = val;
                                self.grid.cells_vec[cell.row()][cell.col()].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row()][cell.col()].valid = false;
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
        if let Some(Value::Oper(Some(box1), Some(box2), oper)) = func.clone() {
            if let (Value::Const(val1), Value::Const(val2)) = (*box1, *box2) {
                update_edges(&mut self.grid, cell.clone(), func.clone(), true); //debug check //add break edges
                // change cell's parameters here
                let sequence = get_sequence(&self.grid, cell.clone(), func.clone());
                update_grid(&mut self.grid, sequence.clone());
            }
            else {
                update_edges(&mut self.grid, cell.clone(), func.clone(),true);
                if (has_cycle(&mut self.grid, cell.clone(), func.clone())) {
                    update_edges(&mut self.grid, cell.clone(), func.clone(),false);
                    return Status::CircularDependency;
                }
                // change cell's parameters here
                let sequence = get_sequence(&self.grid, cell.clone(), func.clone());
                update_grid(&mut self.grid, sequence.clone());
            }
        }
        Status::Success
    }
    ///Takes command from frontend, calls the Parser, and sends the decoded command to execute function
    pub fn process_command(&mut self, rows: usize, columns: usize, cmd: String) -> Status {
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
                return self.execute(Value::Cell(col, row), Value::Oper(box1, box2, op));
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
