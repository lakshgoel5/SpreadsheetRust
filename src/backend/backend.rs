use crate::backend::graph::Cell;
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
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    ///Function to initialize grid. Arguments are size of grid.
    pub fn new(rows: usize, columns: usize) -> Self {
        Grid {
            rows,
            columns,
            cells: vec![vec![Cell::new(0); columns]; rows],
        }
    }
    pub fn get_row_size(&self) -> usize {
        self.rows
    }
    pub fn get_column_size(&self) -> usize {
        self.columns
    }
    pub fn get_cell(&self, row: usize, column: usize) -> &Cell {
        &self.cells[row][column]
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
    ///Takes command from frontend, calls the Parser, and sends the decoded command to execute function
    // fn process_command(rows: usize, columns: usize, cmd: String) -> Self {}
    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }
}
