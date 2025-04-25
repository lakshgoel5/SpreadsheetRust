use project::extension::backend::backend::*;
#[allow(unused_imports)]
use project::extension::common::{Operation, Value};
use std::fs;

#[test]
fn test_init_backend() {
    let backend = Backend::init_backend(10, 15);
    let grid = backend.get_grid();
    assert_eq!(grid.get_row_size(), 10 + 1);
    assert_eq!(grid.get_column_size(), 15 + 1);
    assert_eq!(grid.get_node_value(9, 14), Some(0));
}

#[test]
#[should_panic(expected = "Expected a Cell value")]
fn test_init_backend1() {
    let backend = Backend::init_backend(10, 15);
    let val = Value::Const(5);
    backend.get_node_value(val);
}

#[test]
fn test_grid_new() {
    let grid = Grid::new(10, 15);
    assert_eq!(grid.get_row_size(), 10);
    assert_eq!(grid.get_column_size(), 15);
}

#[test]
fn test_valgrid_creation() {
    let backend = Backend::init_backend(5, 5);
    let valgrid = backend.get_valgrid();
    assert_eq!(valgrid.rows, 6);
    assert_eq!(valgrid.columns, 6);
}

#[test]
fn test_process_command_assign_value() {
    let mut backend = Backend::init_backend(10, 10);
    let status = backend.process_command(10, 10, "A1=5".to_string());
    assert_eq!(status, Status::Success);
    assert_eq!(backend.get_node_value(Value::Cell(1, 1)), Some(5));
}

#[test]
fn test_process_command_invalid() {
    let mut backend = Backend::init_backend(10, 10);
    let status = backend.process_command(10, 10, "invalid command".to_string());
    assert_eq!(status, Status::UnrecognizedCmd);
}

#[test]
fn test_process_command_sum() {
    let mut backend = Backend::init_backend(10, 10);
    // Assign values to cells
    backend.process_command(10, 10, "A1=5".to_string());
    backend.process_command(10, 10, "A2=10".to_string());
    backend.process_command(10, 10, "A3=15".to_string());

    // Test SUM function
    let status = backend.process_command(10, 10, "B1=SUM(A1:A3)".to_string());
    assert_eq!(status, Status::Success);
    assert_eq!(backend.get_node_value(Value::Cell(1, 2)), Some(30));
}

#[test]
fn test_process_command_avg() {
    let mut backend = Backend::init_backend(10, 10);
    // Assign values to cells
    backend.process_command(10, 10, "A1=6".to_string());
    backend.process_command(10, 10, "A2=12".to_string());
    backend.process_command(10, 10, "A3=18".to_string());

    // Test AVG function

    let status = backend.process_command(10, 10, "B1=AVG(A1:A3)".to_string());
    assert_eq!(status, Status::Success);
    assert_eq!(backend.get_node_value(Value::Cell(1, 2)), Some(12));
}

#[test]
fn test_process_command_arithmetic() {
    let mut backend = Backend::init_backend(10, 10);
    // Assign values to cells
    backend.process_command(10, 10, "A1=5".to_string());
    backend.process_command(10, 10, "A2=10".to_string());

    // Test arithmetic operations
    backend.process_command(10, 10, "B1=A1+A2".to_string()); // Addition
    backend.process_command(10, 10, "B2=A2-A1".to_string()); // Subtraction
    backend.process_command(10, 10, "B3=A1*A2".to_string()); // Multiplication
    backend.process_command(10, 10, "B4=A2/A1".to_string()); // Division

    assert_eq!(backend.get_node_value(Value::Cell(1, 2)), Some(15)); // 5+10
    assert_eq!(backend.get_node_value(Value::Cell(2, 2)), Some(5)); // 10-5
    assert_eq!(backend.get_node_value(Value::Cell(3, 2)), Some(50)); // 5*10
    assert_eq!(backend.get_node_value(Value::Cell(4, 2)), Some(2)); // 10/5
}

#[test]
fn test_process_command_stdev() {
    let mut backend = Backend::init_backend(10, 10);
    // Assign values to cells
    backend.process_command(10, 10, "A1=5".to_string());
    backend.process_command(10, 10, "A2=10".to_string());
    backend.process_command(10, 10, "A3=15".to_string());

    // Test SUM function
    let status = backend.process_command(10, 10, "B1=STDEV(A1:A3)".to_string());
    assert_eq!(status, Status::Success);
    assert_eq!(backend.get_node_value(Value::Cell(1, 2)), Some(4));
}

#[test]
fn test_process_command_disable() {
    let mut backend = Backend::init_backend(10, 10);
    // Assign values to cells
    backend.process_command(10, 10, "A1=5".to_string());
    backend.process_command(10, 10, "A2=10".to_string());
    backend.process_command(10, 10, "A3=15".to_string());

    // Test SUM function
    let status = backend.process_command(10, 10, "disable_output".to_string());
    assert_eq!(status, Status::Success);
}

#[test]
fn test_process_command_up() {
    let mut backend = Backend::init_backend(10, 10);
    // Assign values to cells
    backend.process_command(10, 10, "A1=5".to_string());
    backend.process_command(10, 10, "A2=10".to_string());
    backend.process_command(10, 10, "A3=15".to_string());

    // Test SUM function
    let status = backend.process_command(10, 10, "w".to_string());
    assert_eq!(status, Status::Success);
}

#[test]
fn test_process_command_stdev_none() {
    let mut backend = Backend::init_backend(10, 10);
    // Assign values to cells
    backend.process_command(10, 10, "A1=5".to_string());
    backend.process_command(10, 10, "A2=10".to_string());
    backend.process_command(10, 10, "A3=15".to_string());
    backend.grid.cells_vec[1][1].valid = false;
    // Test SUM function
    let status = backend.process_command(10, 10, "B1=STDEV(A1:A3)".to_string());
    assert_eq!(status, Status::Success);
    assert_eq!(backend.get_node_value(Value::Cell(1, 2)), None);
}

#[test]
fn test_process_command_sleep() {
    let mut backend = Backend::init_backend(10, 10);
    // Assign values to cells
    backend.process_command(10, 10, "A1=1".to_string());
    backend.process_command(10, 10, "A2=10".to_string());
    backend.process_command(10, 10, "A3=15".to_string());

    // Test SUM function
    let status = backend.process_command(10, 10, "B1=SLEEP(A1)".to_string());
    assert_eq!(status, Status::Success);
    assert_eq!(backend.get_node_value(Value::Cell(1, 2)), Some(1));
}

#[test]
fn test_process_command_sleep_none() {
    let mut backend = Backend::init_backend(10, 10);
    // Assign values to cells
    backend.process_command(10, 10, "A1=5".to_string());
    backend.process_command(10, 10, "A2=10".to_string());
    backend.process_command(10, 10, "A3=15".to_string());
    backend.grid.cells_vec[1][1].valid = false;
    // Test SUM function
    let status = backend.process_command(10, 10, "B1=SLEEP(A1)".to_string());
    assert_eq!(status, Status::Success);
}

#[test]
fn test_process_command_sum_none() {
    let mut backend = Backend::init_backend(10, 10);
    // Assign values to cells
    backend.process_command(10, 10, "A1=5".to_string());
    backend.process_command(10, 10, "A2=10".to_string());
    backend.process_command(10, 10, "A3=15".to_string());
    backend.grid.cells_vec[1][1].valid = false;
    // Test SUM function
    let status = backend.process_command(10, 10, "B1=SUM(A1:A3)".to_string());
    assert_eq!(status, Status::Success);
    assert_eq!(backend.get_node_value(Value::Cell(1, 2)), None);
}

#[test]
fn test_process_command_avg_none() {
    let mut backend = Backend::init_backend(10, 10);
    // Assign values to cells
    backend.process_command(10, 10, "A1=6".to_string());
    backend.process_command(10, 10, "A2=12".to_string());
    backend.process_command(10, 10, "A3=18".to_string());

    // Test AVG function
    backend.grid.cells_vec[1][1].valid = false;
    let status = backend.process_command(10, 10, "B1=AVG(A1:A3)".to_string());
    assert_eq!(status, Status::Success);
    assert_eq!(backend.get_node_value(Value::Cell(1, 2)), None);
}

#[test]
fn test_process_command_arithmetic_none() {
    let mut backend = Backend::init_backend(10, 10);
    // Assign values to cells
    backend.process_command(10, 10, "A1=5".to_string());
    backend.process_command(10, 10, "A2=10".to_string());
    // Test arithmetic operations
    backend.grid.cells_vec[1][1].valid = false;
    backend.process_command(10, 10, "B1=A1+A2".to_string()); // Addition
    backend.process_command(10, 10, "B2=A2-A1".to_string()); // Subtraction
    backend.process_command(10, 10, "B3=A1*A2".to_string()); // Multiplication
    backend.process_command(10, 10, "B4=A2/A1".to_string()); // Division

    assert_eq!(backend.get_node_value(Value::Cell(1, 2)), None); // 5+10
    assert_eq!(backend.get_node_value(Value::Cell(2, 2)), None); // 10-5
    assert_eq!(backend.get_node_value(Value::Cell(3, 2)), None); // 5*10
    assert_eq!(backend.get_node_value(Value::Cell(4, 2)), None); // 10/5
}

#[test]
fn test_circular_dependency() {
    let mut backend = Backend::init_backend(10, 10);
    // Create a circular dependency
    backend.process_command(10, 10, "A1=5".to_string());
    backend.process_command(10, 10, "A2=A1+10".to_string());
    let status = backend.process_command(10, 10, "A1=A2+5".to_string());

    assert_eq!(status, Status::CircularDependency);
}

#[test]
fn test_undo_redo() {
    let mut backend = Backend::init_backend(10, 10);

    // Make changes
    backend.process_command(10, 10, "A1=5".to_string());
    assert_eq!(backend.get_node_value(Value::Cell(1, 1)), Some(5));

    backend.process_command(10, 10, "A1=10".to_string());
    assert_eq!(backend.get_node_value(Value::Cell(1, 1)), Some(10));

    // Undo
    backend.process_command(10, 10, "undo".to_string());
    assert_eq!(backend.get_node_value(Value::Cell(1, 1)), Some(5));

    // Redo
    backend.process_command(10, 10, "redo".to_string());
    assert_eq!(backend.get_node_value(Value::Cell(1, 1)), Some(10));
}

#[test]
fn test_serialization() {
    let mut backend = Backend::init_backend(5, 5);
    backend.process_command(5, 5, "A1=42".to_string());

    // Create a temporary file path
    let temp_file = "test_serialization.json";

    // Serialize
    let result = backend.serial(temp_file);
    assert!(result.is_ok());

    // Deserialize
    let loaded_backend = Backend::deserial(temp_file);
    assert!(loaded_backend.is_ok());

    let loaded_backend = loaded_backend.unwrap();
    assert_eq!(loaded_backend.get_node_value(Value::Cell(1, 1)), Some(42));

    // Clean up
    if let Ok(_) = fs::remove_file(temp_file) {
        // File deleted successfully
    }
}

#[test]
fn test_invalid_range() {
    let mut backend = Backend::init_backend(10, 10);
    let status = backend.process_command(10, 10, "A1=SUM(Z1:Z10)".to_string());
    // Z column is beyond our 10 columns, so it should report invalid range
    assert_eq!(status, Status::UnrecognizedCmd);
}

#[test]
fn test_min_max_functions() {
    let mut backend = Backend::init_backend(10, 10);
    // Assign values to cells
    backend.process_command(10, 10, "A1=5".to_string());
    backend.process_command(10, 10, "A2=10".to_string());
    backend.process_command(10, 10, "A3=15".to_string());
    backend.process_command(10, 10, "A4=2".to_string());

    // Test MIN and MAX functions
    backend.process_command(10, 10, "B1=MIN(A1:A4)".to_string());
    backend.process_command(10, 10, "B2=MAX(A1:A4)".to_string());

    assert_eq!(backend.get_node_value(Value::Cell(1, 2)), Some(2)); // MIN
    assert_eq!(backend.get_node_value(Value::Cell(2, 2)), Some(15)); // MAX
}
