use spreadsheet_rust::backend::backend;
use spreadsheet_rust::backend::backend::Backend;

#[test]
fn test_init_backend() {
    let backend = Backend::init_backend(10, 15);
    let grid = backend.get_grid();
    assert_eq!(grid.get_row_size(), 10);
    assert_eq!(grid.get_column_size(), 15);
    assert_eq!(grid.get_cell(9, 14).get_cell_value(), 0)
}
#[test]
fn test_grid_new() {
    let grid = backend::Grid::new(10, 15);
    assert_eq!(grid.get_row_size(), 10);
    assert_eq!(grid.get_column_size(), 15);
}
