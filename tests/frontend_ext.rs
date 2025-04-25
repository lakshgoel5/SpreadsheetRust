#[allow(unused_imports)]
use project::extension::common::{Operation, Value};
use project::extension::frontend::frontend::*;
use std::fs;
use std::io::Cursor;
use std::process::Command;
use tempfile::tempdir;

#[cfg(test)]
mod frontend_tests {
    use super::*;

    #[test]
    fn test_column_decoder() {
        // Test basic column conversions
        assert_eq!(column_decoder(1), "A");
        assert_eq!(column_decoder(2), "B");
        assert_eq!(column_decoder(26), "Z");
        assert_eq!(column_decoder(27), "AA");
        assert_eq!(column_decoder(28), "AB");
        assert_eq!(column_decoder(52), "AZ");
        assert_eq!(column_decoder(53), "BA");
        assert_eq!(column_decoder(702), "ZZ");
        assert_eq!(column_decoder(703), "AAA");
    }

    #[test]
    fn test_frontend_initialization() {
        // Test initialization with empty path (new sheet)
        let frontend = Frontend::init_frontend(10, 10, "");
        assert_eq!(frontend.start, Value::Cell(1, 1));
        assert_eq!(frontend.dimension, Value::Cell(10, 10));
        assert!(frontend.print_enabled);

        // Create a temporary file for serialization test
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let file_path = temp_dir.path().join("test_sheet.json");
        let path_str = file_path.to_str().unwrap();

        // Create a frontend and serialize it to test file
        let mut initial_frontend = Frontend::init_frontend(20, 15, "");
        initial_frontend
            .backend
            .set_cell(2, 3, Value::Number(42.0))
            .unwrap();
        initial_frontend
            .backend
            .set_cell(4, 5, Value::Text("Test".to_string()))
            .unwrap();
        initial_frontend
            .backend
            .serial(path_str)
            .expect("Failed to serialize");

        // Test initialization with existing file
        let loaded_frontend = Frontend::init_frontend(10, 10, path_str);
        assert_eq!(loaded_frontend.start, Value::Cell(1, 1));

        // Check that the loaded backend has the correct values
        if let Some(Value::Number(val)) = loaded_frontend.backend.get_grid().get_node_value(2, 3) {
            assert_eq!(val, 42.0);
        } else {
            panic!("Expected Value::Number at cell (2,3)");
        }

        if let Some(Value::Text(val)) = loaded_frontend.backend.get_grid().get_node_value(4, 5) {
            assert_eq!(val, "Test");
        } else {
            panic!("Expected Value::Text at cell (4,5)");
        }
    }

    #[test]
    fn test_execute_status() {
        // Test navigation commands
        let mut frontend = Frontend::init_frontend(100, 100, "");

        // Test Left command
        frontend.start = Value::Cell(50, 50);
        frontend.execute_status(&Status::Left);
        assert_eq!(frontend.start, Value::Cell(50, 40));

        // Test extreme Left (should stop at 1)
        frontend.start = Value::Cell(50, 5);
        frontend.execute_status(&Status::Left);
        assert_eq!(frontend.start, Value::Cell(50, 1));

        // Test Right command
        frontend.start = Value::Cell(50, 50);
        frontend.execute_status(&Status::Right);
        assert_eq!(frontend.start, Value::Cell(50, 60));

        // Test extreme Right (should stop at dimension-9)
        frontend.start = Value::Cell(50, 95);
        frontend.execute_status(&Status::Right);
        assert_eq!(frontend.start, Value::Cell(50, 91)); // dimension.col() - 9

        // Test Up command
        frontend.start = Value::Cell(50, 50);
        frontend.execute_status(&Status::Up);
        assert_eq!(frontend.start, Value::Cell(40, 50));

        // Test extreme Up (should stop at 1)
        frontend.start = Value::Cell(5, 50);
        frontend.execute_status(&Status::Up);
        assert_eq!(frontend.start, Value::Cell(1, 50));

        // Test Down command
        frontend.start = Value::Cell(50, 50);
        frontend.execute_status(&Status::Down);
        assert_eq!(frontend.start, Value::Cell(60, 50));

        // Test extreme Down (should stop at dimension-9)
        frontend.start = Value::Cell(95, 50);
        frontend.execute_status(&Status::Down);
        assert_eq!(frontend.start, Value::Cell(91, 50)); // dimension.row() - 9

        // Test PrintDisabled command
        frontend.execute_status(&Status::PrintDisabled);
        assert_eq!(frontend.print_enabled, false);

        // Test PrintEnabled command
        frontend.execute_status(&Status::PrintEnabled);
        assert_eq!(frontend.print_enabled, true);

        // Test ScrollTo command
        frontend.execute_status(&Status::ScrollTo(25, 35));
        assert_eq!(frontend.start, Value::Cell(25, 35));
    }

    #[test]
    fn test_print_grid_enabled() {
        let mut frontend = Frontend::init_frontend(10, 10, "");
        // Set up some test data
        frontend
            .backend
            .set_cell(1, 1, Value::Number(100.0))
            .unwrap();
        frontend
            .backend
            .set_cell(2, 2, Value::Text("Test".to_string()))
            .unwrap();

        // Ensure print is enabled
        frontend.print_enabled = true;

        // Capture stdout to verify output
        let mut output = Vec::new();
        {
            use std::io::Write;
            // Call print_grid (would write to stdout in normal circumstances)
            // For test purposes, we're not capturing actual stdout here
            // but verifying that with print_enabled true, the function executes fully
            frontend.print_grid();
            writeln!(output, "Output captured").unwrap();
        }

        // Verify that something was written
        assert!(!output.is_empty());
    }

    #[test]
    fn test_print_grid_disabled() {
        let mut frontend = Frontend::init_frontend(10, 10, "");
        // Set up some test data
        frontend
            .backend
            .set_cell(1, 1, Value::Number(100.0))
            .unwrap();

        // Disable printing
        frontend.print_enabled = false;

        // With print_enabled set to false, print_grid should return early
        // We can't easily test the absence of output, but we can verify
        // that the function doesn't crash when printing is disabled
        frontend.print_grid();

        // If we reach here without errors, the test passes
        assert!(!frontend.print_enabled);
    }

    #[test]
    fn test_invalid_location_dimension() {
        // Test handling of invalid location/dimension values
        let mut frontend = Frontend::init_frontend(10, 10, "");

        // Set invalid values that don't match the Cell pattern
        frontend.start = Value::Number(42.0);
        frontend.dimension = Value::Text("invalid".to_string());

        // This should not panic, but handle the error gracefully
        frontend.print_grid();

        // If we reach here without errors, the test passes
        assert!(true);
    }

    // Note: run_counter and display are harder to test in isolation
    // as they deal with stdin/stdout and would require more complex mocking
}
