use spreadsheet_rust::src::backend::graph::Cell;

#[test]
fn test_cell_creation() {
    let cell = Cell::new(10);
    assert_eq!(cell.node_value, 10);
    assert_eq!(cell.dependents.len(), 0);
    assert!(!cell.visited);
    assert!(!cell.valid);
    assert!(cell.function.is_none());
}
