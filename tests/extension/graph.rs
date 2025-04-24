use spreadsheet_rust::backend::graph::Node;

#[test]
fn test_cell_creation() {
    let cell = Node::new(10);
    assert_eq!(cell.node_value, 10);
    assert_eq!(cell.dependents.len(), 0);
    assert!(!cell.visited);
    assert!(!cell.valid);
    assert!(cell.function.is_none());
}
