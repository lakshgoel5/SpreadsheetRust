use spreadsheet_rust::src::backend::backend;

#[test]
fn test_init_backend() {
    backend::init_backend(10, 15);
    let grid = backend.get_grid();
    assert_eq!(grid.rows, 10);
    assert_eq!(grid.columns, 15);
    assert_eq!(grid.cells.len(), 10);
    assert_eq!(grid.cells[0].len(), 15);
    assert_eq!(grid.cells[0][0].node_value, 0);
}

fn test_grid_new() {
    let grid = backend::Grid::new(10, 15);
    assert_eq!(grid.rows, 10);
    assert_eq!(grid.columns, 15);
    assert_eq!(grid.cells.len(), 10);
}
