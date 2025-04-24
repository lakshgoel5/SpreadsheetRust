#![allow(dead_code)]
use std::fs;
const UNDO_LIMIT: usize = 1000;
use crate::backend::functions::*;
use crate::backend::graph::Node;
use crate::backend::graph::get_sequence;
use crate::backend::graph::has_cycle;
use crate::backend::graph::update_edges;
use crate::common::{Operation, Value};
use crate::parser::*;
use serde::{Deserialize, Serialize};
//init_backend(r,c) -> generate a grid of all nodes : returns void
//execute(value::cell, value::oper) -> update_edges(Node, value::oper), hasCycle(Box<>, value::cell), get_sequence(Box<>, value::cell), update_grid(sequence) -> return status
//update_grid(sequence) -> loop assign to Node = <functions>(Box<>, value::oper -> return bool
//process_command(r,c, string, value::Cell) -> parser, execute(value::cell, value::oper): return status
//get_value(value::cell): returns a cell_value
/// Control Unit for data processing and updating values in Spreadsheeet.
/// The `Grid` struct is designed to store and manage a grid of `Cell` objects.
///Data structure to represent sheet
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Grid {
    rows: usize,
    columns: usize,
    cells_vec: Vec<Vec<Node>>,
}

/// Status codes returned by commands executed on the spreadsheet.
///
/// These status values provide feedback about the result of operations
/// performed on the spreadsheet, including navigation, cell updates,
/// and error conditions.
#[derive(PartialEq)]
pub enum Status {
    /// Command executed successfully
    Success,
    /// Invalid range specified in a formula or command
    InvalidRange,
    /// Command not recognized by the parser
    UnrecognizedCmd,
    /// Invalid row or column index provided
    InvalidRowColumn,
    /// A circular dependency was detected in cell formulas
    CircularDependency,
    /// Output printing has been enabled
    PrintEnabled,
    /// Output printing has been disabled
    PrintDisabled,
    /// Command to scroll to a specific cell position (row, column)
    ScrollTo(usize, usize),
    /// Navigation command to move up
    Up,
    /// Navigation command to move down
    Down,
    /// Navigation command to move left
    Left,
    /// Navigation command to move right
    Right,
    /// Command to quit the application
    Quit,
    Web(String),
    WebStart,
}

impl Grid {
    /// Creates a new grid with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `rows` - Number of rows in the grid
    /// * `columns` - Number of columns in the grid
    ///
    /// # Returns
    ///
    /// A new Grid instance with all cells initialized to zero
    pub fn new(rows: usize, columns: usize) -> Self {
        Grid {
            rows,
            columns,
            cells_vec: vec![vec![Node::new(0); columns]; rows],
        }
    }

    /// Returns the number of rows in the grid.
    pub fn get_row_size(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns in the grid.
    pub fn get_column_size(&self) -> usize {
        self.columns
    }

    /// Returns a mutable reference to the node at the specified position.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index (0-based)
    /// * `column` - The column index (0-based)
    ///
    /// # Returns
    ///
    /// A mutable reference to the Node at the specified position
    pub fn get_node(&mut self, row: usize, column: usize) -> &mut Node {
        &mut self.cells_vec[row][column]
    }

    /// Returns the value of the node at the specified position.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index (0-based)
    /// * `column` - The column index (0-based)
    ///
    /// # Returns
    ///
    /// The value of the cell if valid, or None if the cell has an error
    pub fn get_node_value(&self, row: usize, column: usize) -> Option<isize> {
        self.cells_vec[row][column].get_node_value()
    }
    pub fn get_node_ref(&self, row: usize, col: usize) -> &Node {
        &self.cells_vec[row][col]
    }
    // pub fn get_node_mut(&mut self, row: usize, column: usize) -> &mut Node {
    //     &mut self.cells_vec[row][column]
    // }
}

/// Representation of the grid's values for external use.
///
/// This struct provides a simplified view of the grid, containing just the
/// numeric values of cells without the dependencies and formulas
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Valgrid {
    /// Number of rows in the value grid
    pub rows: usize,
    /// Number of columns in the value grid
    pub columns: usize,
    pub cells: Vec<Vec<Option<isize>>>,
}

/// Backend controller for the spreadsheet application.
///
/// Manages the grid data structure and provides methods for processing commands,
/// handling cell updates, and evaluating formulas while detecting circular dependencies.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Backend {
    grid: Grid,
    undo_stack: Vec<Grid>,
    redo_stack: Vec<Grid>,
}

impl Backend {
    /// Creates a new Backend instance with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `rows` - Number of rows in the grid
    /// * `columns` - Number of columns in the grid
    ///
    /// # Returns
    ///
    /// A new Backend instance with an initialized grid (adding 1 to dimensions for 1-based indexing)
    pub fn init_backend(rows: usize, columns: usize) -> Self {
        Backend {
            grid: Grid::new(rows + 1, columns + 1),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    /// Returns the value of a cell.
    ///
    /// # Arguments
    ///
    /// * `cell` - A Value::Cell containing row and column coordinates
    ///
    /// # Returns
    ///
    /// The value of the specified cell if valid, or None if the cell has an error
    ///
    /// # Panics
    ///
    /// Panics if the provided Value is not a Cell
    pub fn get_node_value(&self, cell: Value) -> Option<isize> {
        match cell {
            Value::Cell(row, col) => self.grid.get_node_value(row, col),
            _ => panic!("Expected a Cell value"),
        }
    }

    /// Returns a representation of the grid with all current cell values.
    ///
    /// Creates a Valgrid structure that contains just the calculated values
    /// of all cells, without formulas and dependencies.
    ///
    /// # Returns
    ///
    /// A Valgrid instance containing the current values of all cells
    pub fn get_valgrid(&self) -> Valgrid {
        Valgrid {
            rows: self.grid.get_row_size(),
            columns: self.grid.get_column_size(),
            cells: self
                .grid
                .cells_vec
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|cell| if cell.valid { Some(cell.node_value) } else { None })
                        .collect()
                })
                .collect(),
        }
    }

    /// Updates cell values based on a topological sort of dependencies.
    ///
    /// # Arguments
    ///
    /// * `sequence` - Vector of cells in topological order to be updated
    ///
    /// Evaluates each cell's formula in the sequence, updating values and
    /// marking cells as valid or invalid based on the result.
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
                                self.grid.cells_vec[cell.row()][cell.col()].node_value = val;
                                self.grid.cells_vec[cell.row()][cell.col()].valid = true;
                            }
                            None => {
                                self.grid.cells_vec[cell.row()][cell.col()].valid = false;
                            }
                        }
                    }
                    Operation::Min => {
                        let min = min_function(&mut self.grid, cell.row(), cell.col());
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
                        let max = max_function(&mut self.grid, cell.row(), cell.col());
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
                        let avg = avg_function(&mut self.grid, cell.row(), cell.col());
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
                        let std_dev = std_dev_function(&mut self.grid, cell.row(), cell.col());
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
                        let ans = add(&mut self.grid, cell.row(), cell.col());
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
                        let ans = sub(&mut self.grid, cell.row(), cell.col());
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
                        let ans = mul(&mut self.grid, cell.row(), cell.col());
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
                        let ans = div(&mut self.grid, cell.row(), cell.col());
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
                        let ans = slp(&mut self.grid, cell.row(), cell.col());
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
                        let ans = cons(&mut self.grid, cell.row(), cell.col());
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

    /// Processes cell updates while checking for circular dependencies.
    ///
    /// # Arguments
    ///
    /// * `cell` - The target cell to update
    /// * `func` - The formula/operation to apply to the cell
    ///
    /// # Returns
    ///
    /// A Status indicating success or the type of error that occurred
    fn execute(&mut self, cell: Value, func: Option<Value>) -> Status {
        //I want that if func has first and second box as value::const type, then just update graph and evaluate expression by sending Operation as well
        if let Some(Value::Oper(Some(box1), Some(box2), _oper)) = func.clone() {
            if let (Value::Const(_val1), Value::Const(_val2)) = (*box1, *box2) {
                update_edges(&mut self.grid, &cell, &func, true); //debug check //add break edges
                // change cell's parameters here
                let node = self.grid.get_node(cell.row(), cell.col());
                node.function = func.clone();
                let sequence = get_sequence(&mut self.grid, &cell);
                self.update_grid(sequence);
            } else {
                update_edges(&mut self.grid, &cell, &func, true);
                if has_cycle(&mut self.grid, &cell) {
                    update_edges(&mut self.grid, &cell, &func, false);
                    return Status::CircularDependency;
                }
                // change cell's parameters here
                let node = self.grid.get_node(cell.row(), cell.col());
                node.function = func.clone();
                let sequence = get_sequence(&mut self.grid, &cell);
                self.update_grid(sequence);
            }
        }
        Status::Success
    }

    /// Processes a command string from the frontend.
    ///
    /// # Arguments
    ///
    /// * `rows` - Maximum number of rows in the spreadsheet
    /// * `columns` - Maximum number of columns in the spreadsheet
    /// * `cmd` - The command string to process
    ///
    /// # Returns
    ///
    /// A Status indicating the result of the command
    pub fn process_command(&mut self, rows: usize, columns: usize, cmd: String) -> Status {
        match parser::validate(&cmd, &columns, &rows) {
            Some((None, Some(Value::Oper(None, None, op)))) => match op {
                Operation::EnableOutput => Status::PrintEnabled,
                Operation::DisableOutput => Status::PrintDisabled,
                Operation::Left => Status::Left,
                Operation::Right => Status::Right,
                Operation::Up => Status::Up,
                Operation::Down => Status::Down,
                Operation::Quit => Status::Quit,
                Operation::Web(path) => Status::Web(path),
                Operation::WebStart => Status::WebStart,
                Operation::Save(path) => {
                    if let Err(_) = self.serial(&path) {
                        return Status::UnrecognizedCmd;
                    }
                    Status::Success
                }
                Operation::Undo => {
                    if let Some(prev_grid) = self.undo_stack.pop() {
                        self.redo_stack.push(self.grid.clone());
                        self.grid = prev_grid;
                        Status::Success
                    } else {
                        Status::UnrecognizedCmd
                    }
                },
                Operation::Redo => {
                    if let Some(next_grid) = self.redo_stack.pop() {
                        self.undo_stack.push(self.grid.clone());
                        if self.undo_stack.len() > UNDO_LIMIT {
                            self.undo_stack.remove(0); // drop oldest
                        }
                        self.grid = next_grid;
                        Status::Success
                    } else {
                        Status::UnrecognizedCmd
                    }
                },
                _ => Status::UnrecognizedCmd,
            },
            Some((
                Some(Value::Cell(col, row)),
                Some(Value::Oper(None, None, Operation::ScrollTo)),
            )) => Status::ScrollTo(col, row),
            Some((Some(Value::Cell(col, row)), Some(Value::Oper(box1, box2, op)))) => {
                // change here
                // either have to change parser or change the inside parts of box1 and box2
                self.undo_stack.push(self.grid.clone());
                if self.undo_stack.len() > UNDO_LIMIT {
                    self.undo_stack.remove(0);
                }
                self.redo_stack.clear(); // clear redo stack on new action
                self.execute(Value::Cell(col, row), Some(Value::Oper(box1, box2, op)))
            }
            _ => Status::UnrecognizedCmd,
        }
    }

    /// Returns a reference to the underlying grid.
    ///
    /// # Returns
    ///
    /// An immutable reference to the Grid
    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }

    pub fn serial(&self, path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Serialization error: {}", e))?;
        std::fs::write(path, json).map_err(|e| format!("File write error: {}", e))
    }

    pub fn deserial(path: &str) -> Result<Self, String> {
        let json = fs::read_to_string(path).map_err(|e| format!("File read error: {}", e))?;
        serde_json::from_str(&json).map_err(|e| format!("Deserialization error: {}", e))
    }

    pub fn deserial_text(text: String) -> Result<Self, String> {
        serde_json::from_str(&text).map_err(|e| format!("Deserialization error: {}", e))
    }
}
