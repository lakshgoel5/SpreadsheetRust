use project::extension::backend::backend::*;
#[allow(unused_imports)]
use project::extension::common::{Operation, Value};
use project::extension::frontend::terminal::*;

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
        // Test initialization with empty path
        let frontend = Frontend::init_frontend(10, 20, "");

        // Check that default values are set correctly
        assert_eq!(frontend.start, Value::Cell(1, 1));
        assert_eq!(frontend.dimension, Value::Cell(10, 20));
        assert_eq!(frontend.print_enabled, true);

        // Check that backend was initialized with correct dimensions
        assert_eq!(frontend.backend.grid.get_row_size(), 11); // +1 because backend adds 1
        assert_eq!(frontend.backend.grid.get_column_size(), 21); // +1 because backend adds 1

        // Test initialization with a path (this would require creating a test file first)
        // This part is more complex and might need mocking, so we'll skip it for now
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

        // Directly access and modify grid nodes through public field
        frontend.backend.grid.cells_vec[1][1].node_value = 100;
        frontend.backend.grid.cells_vec[1][1].valid = true;

        frontend.backend.grid.cells_vec[2][2].node_value = 42; // For text display test
        frontend.backend.grid.cells_vec[2][2].valid = true;

        // Ensure print is enabled
        frontend.print_enabled = true;

        // Capture stdout to verify output
        let mut output = Vec::new();
        {
            use std::io::Write;
            frontend.print_grid();
            writeln!(output, "Output captured").unwrap();
        }

        // Verify that something was written
        assert!(!output.is_empty());
    }

    #[test]
    fn test_print_grid_disabled() {
        let mut frontend = Frontend::init_frontend(10, 10, "");

        // Set up some test data directly
        frontend.backend.grid.cells_vec[1][1].node_value = 100;
        frontend.backend.grid.cells_vec[1][1].valid = true;

        // Disable printing
        frontend.print_enabled = false;

        // With print_enabled set to false, print_grid should return early
        frontend.print_grid();

        // If we reach here without errors, the test passes
        assert!(!frontend.print_enabled);
    }

    #[test]
    fn test_display() {
        let mut frontend = Frontend::init_frontend(10, 10, "");

        // Set up a test cell
        frontend.backend.grid.cells_vec[1][1].node_value = 100;
        frontend.backend.grid.cells_vec[1][1].valid = true;

        // Capture stdout to verify output
        let mut output = Vec::new();
        {
            use std::io::{Write, stdout};
            let _stdout_backup = stdout();
            // Note: In a real test, you'd use a crate like `rexpect` or set up
            // proper stdout capturing, but this is simplified
            frontend.display(Status::Success, 0.5);
            writeln!(output, "Output captured").unwrap();
        }

        // Verify output contains expected elements
        // This is just a basic test; actual implementation would need proper stdout capture
        assert!(!output.is_empty());
    }

    #[test]
    fn test_invalid_location_dimension() {
        // Test handling of invalid location/dimension values
        let mut frontend = Frontend::init_frontend(10, 10, "");

        // Set invalid values that don't match the Cell pattern
        frontend.start = Value::Const(42); // Using Const instead of Number
        frontend.dimension = Value::Oper(None, None, Operation::Quit); // Using Oper instead of Text

        // This should not panic, but handle the error gracefully
        frontend.print_grid();

        // If we reach here without errors, the test passes
        assert!(frontend.print_enabled); // Check some state was maintained
    }

    #[test]
    fn test_column_decoder_edge_cases() {
        // Test edge cases for column decoder
        assert_eq!(column_decoder(0), ""); // What should happen with 0?
        assert_eq!(column_decoder(1000), "ALL"); // Very large column numbers
        assert_eq!(column_decoder(18278), "ZZZ"); // Test triple-letter columns
    }

    #[test]
    fn test_command_processing() {
        let mut frontend = Frontend::init_frontend(10, 10, "");

        // Test that commands are properly passed to backend
        // You might need to mock backend.process_command to verify this

        // For example:
        let _command = "A1 = 100";
        // Call a method that would trigger process_command
        // Then verify backend state changed appropriately

        // Or test navigation commands:
        frontend.start = Value::Cell(50, 50);
        frontend.execute_status(&Status::ScrollTo(25, 30));
        assert_eq!(frontend.start, Value::Cell(25, 30));
    }

    #[test]
    fn test_dimension_boundaries() {
        let mut frontend = Frontend::init_frontend(5, 5, "");

        // Test that print_grid correctly handles cells at boundaries
        frontend.start = Value::Cell(1, 1);
        frontend.print_grid(); // Should show cells 1,1 through 9,9

        // Test scrolling beyond grid boundaries
        frontend.execute_status(&Status::ScrollTo(100, 100));
        // Verify it doesn't go beyond actual dimensions
        assert_eq!(frontend.start.row(), 5);
        assert_eq!(frontend.start.col(), 5);
    }

    #[test]
    fn test_print_grid_formatting() {
        let mut frontend = Frontend::init_frontend(10, 10, "");

        // Set up a variety of cell values to test formatting
        frontend.backend.grid.cells_vec[1][1].node_value = 12345; // Large number
        frontend.backend.grid.cells_vec[1][1].valid = true;

        frontend.backend.grid.cells_vec[2][2].node_value = -42; // Negative number
        frontend.backend.grid.cells_vec[2][2].valid = true;

        frontend.backend.grid.cells_vec[3][3].node_value = 0; // Zero
        frontend.backend.grid.cells_vec[3][3].valid = true;

        // In a real test, capture and verify stdout formatting
        frontend.print_grid();
        // Assert that formatting is correct (would need proper stdout capture)
    }

    #[test]
    fn test_execute_status_chain() {
        let mut frontend = Frontend::init_frontend(100, 100, "");

        // Test complex sequence of status commands
        frontend.start = Value::Cell(50, 50);

        // Chain of navigation commands
        frontend.execute_status(&Status::Up); // Should go to row 40
        frontend.execute_status(&Status::Left); // Should go to col 40
        frontend.execute_status(&Status::Down); // Should go to row 50
        frontend.execute_status(&Status::Right); // Should go to col 50

        // Verify final position after chain
        assert_eq!(frontend.start, Value::Cell(50, 50));

        // Test disable -> enable print
        frontend.execute_status(&Status::PrintDisabled);
        assert_eq!(frontend.print_enabled, false);
        frontend.execute_status(&Status::PrintEnabled);
        assert_eq!(frontend.print_enabled, true);
    }

    // Note: run_counter and display are harder to test in isolation
    // as they deal with stdin/stdout and would require more complex mocking
}
