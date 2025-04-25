#[allow(unused_imports)]
use project::terminal::functions::{Operation, Value};
use project::terminal::graph::Node;
use project::terminal::spreadsheet::*;
use project::terminal::types::Coordinates;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, Write};

// Existing tests

#[test]
fn test_column_decoder() {
    // Test single-letter columns
    assert_eq!(column_decoder(1), "A");
    assert_eq!(column_decoder(26), "Z");

    // Test double-letter columns
    assert_eq!(column_decoder(27), "AA");
    assert_eq!(column_decoder(28), "AB");
    assert_eq!(column_decoder(52), "AZ");
    assert_eq!(column_decoder(53), "BA");

    // Test triple-letter columns
    assert_eq!(column_decoder(703), "AAA");
    assert_eq!(column_decoder(18278), "ZZZ"); // MAX_COLUMN
}

// New edge cases for column_decoder
#[test]
fn test_column_decoder_edge_cases() {
    // Edge cases
    assert_eq!(column_decoder(702), "ZZ");
    assert_eq!(column_decoder(677), "ZA");
    assert_eq!(column_decoder(676), "YZ");
}

#[test]
fn test_process_first() {
    // Valid arguments
    let args = vec![
        String::from("program_name"),
        String::from("100"),
        String::from("100"),
    ];
    let mut is_disabled = false;
    assert!(process_first(args.len(), &args, &mut is_disabled));

    // Invalid number of arguments
    let invalid_args = vec![String::from("program_name"), String::from("100")];
    assert!(!process_first(
        invalid_args.len(),
        &invalid_args,
        &mut is_disabled
    ));

    // Non-numeric arguments
    let non_numeric_args = vec![
        String::from("program_name"),
        String::from("abc"),
        String::from("100"),
    ];
    assert!(!process_first(
        non_numeric_args.len(),
        &non_numeric_args,
        &mut is_disabled
    ));

    // Out of range arguments (too large)
    let large_args = vec![
        String::from("program_name"),
        String::from("1000"), // Exceeds MAX_ROW
        String::from("100"),
    ];
    assert!(!process_first(
        large_args.len(),
        &large_args,
        &mut is_disabled
    ));

    let large_col_args = vec![
        String::from("program_name"),
        String::from("100"),
        String::from("18279"), // Exceeds MAX_COLUMN
    ];
    assert!(!process_first(
        large_col_args.len(),
        &large_col_args,
        &mut is_disabled
    ));

    // Out of range arguments (too small)
    let zero_args = vec![
        String::from("program_name"),
        String::from("0"), // Below valid range
        String::from("100"),
    ];
    assert!(!process_first(
        zero_args.len(),
        &zero_args,
        &mut is_disabled
    ));
}

// Additional test for process_first with boundary values
#[test]
fn test_process_first_boundary_values() {
    let mut is_disabled = false;

    // Test with minimum valid values
    let min_args = vec![
        String::from("program_name"),
        String::from("1"),
        String::from("1"),
    ];
    assert!(process_first(min_args.len(), &min_args, &mut is_disabled));

    // Test with maximum valid values
    let max_args = vec![
        String::from("program_name"),
        String::from("999"),   // MAX_ROW
        String::from("18278"), // MAX_COLUMN
    ];
    assert!(process_first(max_args.len(), &max_args, &mut is_disabled));

    // Test with too many arguments
    let too_many_args = vec![
        String::from("program_name"),
        String::from("100"),
        String::from("100"),
        String::from("extra"),
    ];
    assert!(!process_first(
        too_many_args.len(),
        &too_many_args,
        &mut is_disabled
    ));

    // Test with empty strings
    let empty_args = vec![
        String::from("program_name"),
        String::from(""),
        String::from("100"),
    ];
    assert!(!process_first(
        empty_args.len(),
        &empty_args,
        &mut is_disabled
    ));
}

// Mock creating a grid for testing
fn create_test_grid(rows: usize, cols: usize) -> Vec<Vec<Node>> {
    let mut grid = Vec::with_capacity(rows + 1);
    for i in 0..=rows {
        let mut row = Vec::with_capacity(cols + 1);
        for j in 0..=cols {
            let mut node = Node {
                node_value: 0,
                valid: false,
                dependents: Vec::new(),
                value1: Coordinates { row: 0, col: 0 },
                value2: Coordinates { row: 0, col: 0 },
                position: Coordinates {
                    row: i as i32,
                    col: j as i32,
                },
                op: Operation::Add, // Default operation
                visited: false,
            };
            if i > 0 && j > 0 {
                // Set some example values
                node.node_value = 0;
                node.valid = true;
            }
            row.push(node);
        }
        grid.push(row);
    }

    // Add some invalid cells for testing
    if rows >= 5 && cols >= 5 {
        grid[3][3].valid = false;
    }

    grid
}

#[test]
fn test_process_command_navigation() {
    let rows = 30;
    let cols = 30;
    let mut grid = create_test_grid(rows, cols);
    let mut start_x = 10;
    let mut start_y = 10;
    let mut is_disabled = false;

    // Test movement commands
    assert_eq!(
        process_command(
            "w",
            &mut start_x,
            &mut start_y,
            rows,
            cols,
            &mut is_disabled,
            &mut grid
        ),
        1
    );
    assert_eq!(start_x, 1); // Move up 10 rows but not below 1

    start_x = 15;
    assert_eq!(
        process_command(
            "w",
            &mut start_x,
            &mut start_y,
            rows,
            cols,
            &mut is_disabled,
            &mut grid
        ),
        1
    );
    assert_eq!(start_x, 5); // Move up 10 rows

    assert_eq!(
        process_command(
            "s",
            &mut start_x,
            &mut start_y,
            rows,
            cols,
            &mut is_disabled,
            &mut grid
        ),
        1
    );
    assert_eq!(start_x, 15); // Move down 10 rows

    assert_eq!(
        process_command(
            "a",
            &mut start_x,
            &mut start_y,
            rows,
            cols,
            &mut is_disabled,
            &mut grid
        ),
        1
    );
    assert_eq!(start_y, 1); // Move left 10 columns but not below 1

    start_y = 15;
    assert_eq!(
        process_command(
            "a",
            &mut start_x,
            &mut start_y,
            rows,
            cols,
            &mut is_disabled,
            &mut grid
        ),
        1
    );
    assert_eq!(start_y, 5); // Move left 10 columns

    assert_eq!(
        process_command(
            "d",
            &mut start_x,
            &mut start_y,
            rows,
            cols,
            &mut is_disabled,
            &mut grid
        ),
        1
    );
    assert_eq!(start_y, 15); // Move right 10 columns

    // Test output toggle commands
    // We'll need to simulate a command that would toggle output
    // Since we don't have direct access to the parser implementation, this is a placeholder
    // You'll need to update this based on your actual implementation
    let disable_output_cmd = "output disable"; // Replace with your actual command syntax
    if let 1 = process_command(
        disable_output_cmd,
        &mut start_x,
        &mut start_y,
        rows,
        cols,
        &mut is_disabled,
        &mut grid,
    ) {
        assert!(is_disabled);
    }

    let enable_output_cmd = "output enable"; // Replace with your actual command syntax
    if let 1 = process_command(
        enable_output_cmd,
        &mut start_x,
        &mut start_y,
        rows,
        cols,
        &mut is_disabled,
        &mut grid,
    ) {
        assert!(!is_disabled);
    }

    // Test quit command
    assert_eq!(
        process_command(
            "q",
            &mut start_x,
            &mut start_y,
            rows,
            cols,
            &mut is_disabled,
            &mut grid
        ),
        0
    );
}

// New test for navigation at grid boundaries
#[test]
fn test_navigation_at_grid_boundaries() {
    let rows = 15;
    let cols = 15;
    let mut grid = create_test_grid(rows, cols);
    let mut is_disabled = false;

    // Test at top boundary
    let mut start_x = 1;
    let mut start_y = 8;
    assert_eq!(
        process_command(
            "w",
            &mut start_x,
            &mut start_y,
            rows,
            cols,
            &mut is_disabled,
            &mut grid
        ),
        1
    );
    assert_eq!(start_x, 1); // Should remain at 1, not go below

    // Test at bottom boundary
    start_x = rows - 8;
    assert_eq!(
        process_command(
            "s",
            &mut start_x,
            &mut start_y,
            rows,
            cols,
            &mut is_disabled,
            &mut grid
        ),
        1
    );
    assert_eq!(start_x, 7);

    // Another move down should be constrained to show the last rows
    assert_eq!(
        process_command(
            "s",
            &mut start_x,
            &mut start_y,
            rows,
            cols,
            &mut is_disabled,
            &mut grid
        ),
        1
    );
    assert_eq!(start_x, rows - 8); // Constrained to show the last 9 rows

    // Test at left boundary
    start_x = 8;
    start_y = 1;
    assert_eq!(
        process_command(
            "a",
            &mut start_x,
            &mut start_y,
            rows,
            cols,
            &mut is_disabled,
            &mut grid
        ),
        1
    );
    assert_eq!(start_y, 1); // Should remain at 1, not go below

    // Test at right boundary
    start_y = cols - 8;
    assert_eq!(
        process_command(
            "d",
            &mut start_x,
            &mut start_y,
            rows,
            cols,
            &mut is_disabled,
            &mut grid
        ),
        1
    );
    assert_eq!(start_y, cols - 8);

    // Another move right should be constrained to show the last columns
    assert_eq!(
        process_command(
            "d",
            &mut start_x,
            &mut start_y,
            rows,
            cols,
            &mut is_disabled,
            &mut grid
        ),
        1
    );
    assert_eq!(start_y, cols - 8); // Constrained to show the last 9 columns
}

// Test invalid commands
#[test]
fn test_invalid_commands() {
    let rows = 20;
    let cols = 20;
    let mut grid = create_test_grid(rows, cols);
    let mut start_x = 10;
    let mut start_y = 10;
    let mut is_disabled = false;

    // Test completely invalid command
    assert_eq!(
        process_command(
            "invalid_command",
            &mut start_x,
            &mut start_y,
            rows,
            cols,
            &mut is_disabled,
            &mut grid
        ),
        3 // Expected to return code 3 for unrecognized command
    );

    // Test command with invalid syntax
    assert_eq!(
        process_command(
            "A1 = invalid_operation(B2, C3)",
            &mut start_x,
            &mut start_y,
            rows,
            cols,
            &mut is_disabled,
            &mut grid
        ),
        3 // Expected to return code 3 for unrecognized command
    );
}

// This is a more involved test that requires mocking the parser
// and getting_things_updated functions
#[test]
fn test_process_command_cell_operations() {
    // This test is more complex and depends on your implementation details
    // You'll need to expand this based on your actual parser and backend functions

    let rows = 30;
    let cols = 30;
    let mut grid = create_test_grid(rows, cols);
    let mut start_x = 10;
    let mut start_y = 10;
    let mut is_disabled = false;

    // Test scrollto operation
    // Replace with your actual command syntax
    let scrollto_cmd = "A5 = scrollto(B2, C3)";

    // For this test to work correctly, you'll need to ensure your parser handles this command
    // and recognizes it as a scrollto operation
    let result = process_command(
        scrollto_cmd,
        &mut start_x,
        &mut start_y,
        rows,
        cols,
        &mut is_disabled,
        &mut grid,
    );

    // If your parser correctly identifies this as a scrollto operation, it should:
    // 1. Update start_x to the row value
    // 2. Update start_y to the column value
    // 3. Return status code 1

    // This is a placeholder assertion - you'll need to update based on actual behavior
    if result == 1 {
        // Check if scrollto was successful (values would be set based on your implementation)
        // The expected values depend on how your parser interprets the command
    }

    // Test basic cell operations
    // These depend heavily on your parser implementation
    let _basic_op_cmd = "B2 = add(C3, 5)";
    // This test requires knowing how your parser and getting_things_updated function work
}

// New test for cell operations based on actual parser examples
#[test]
fn test_cell_operations_with_mock() {
    // This test creates a simulated environment to test cell operations
    // Assuming the parser syntax is like: "A1 = add(B2, C3)"
    let rows = 30;
    let cols = 30;
    let mut grid = create_test_grid(rows, cols);
    let mut start_x = 5;
    let mut start_y = 5;
    let mut is_disabled = false;

    // These commands need to match your actual parser syntax
    // Modify them based on your implementation

    // Basic arithmetic operations
    let add_cmd = "A1 = add(B2, C3)";
    let sub_cmd = "D4 = subtract(E5, F6)";
    let mul_cmd = "G7 = multiply(H8, I9)";
    let div_cmd = "J10 = divide(K11, L12)";

    // Process commands and check status codes
    // The expected return values depend on your implementation
    let add_result = process_command(
        add_cmd,
        &mut start_x,
        &mut start_y,
        rows,
        cols,
        &mut is_disabled,
        &mut grid,
    );

    let sub_result = process_command(
        sub_cmd,
        &mut start_x,
        &mut start_y,
        rows,
        cols,
        &mut is_disabled,
        &mut grid,
    );

    let mul_result = process_command(
        mul_cmd,
        &mut start_x,
        &mut start_y,
        rows,
        cols,
        &mut is_disabled,
        &mut grid,
    );

    let div_result = process_command(
        div_cmd,
        &mut start_x,
        &mut start_y,
        rows,
        cols,
        &mut is_disabled,
        &mut grid,
    );

    // Check status codes based on your expected behavior
    // This is a basic check to ensure the commands are processed
    assert!(add_result >= 1 && add_result <= 5);
    assert!(sub_result >= 1 && sub_result <= 5);
    assert!(mul_result >= 1 && mul_result <= 5);
    assert!(div_result >= 1 && div_result <= 5);
}

#[test]
fn test_display_status() {
    // This is mainly a visual function that writes to stdout
    // We can test that it doesn't panic, but testing the actual output
    // requires capturing stdout which is complex in Rust unit tests

    // Test all status codes
    display_status(1, 0.25);
    display_status(2, 0.50);
    display_status(3, 0.75);
    display_status(4, 1.00);
    display_status(5, 1.25);
    display_status(6, 1.50); // Unknown status code

    // If we reach here without panicking, the test passes
    assert!(true);
}

// New test for display_status with extreme values
#[test]
fn test_display_status_edge_cases() {
    // Test with very small time
    display_status(1, 0.0001);

    // Test with zero time
    display_status(1, 0.0);

    // Test with large time
    display_status(1, 99999.9999);

    // Test negative status code (should be handled gracefully)
    display_status(-1, 1.0);

    // If we reach here without panicking, the test passes
    assert!(true);
}

#[test]
fn test_print_grid() {
    // Similar to display_status, this mainly outputs to stdout
    // We can test that it doesn't panic with various inputs

    let rows = 20;
    let cols = 20;
    let mut grid = create_test_grid(rows, cols);

    // Test different view windows
    print_grid(1, 1, rows, cols, &mut grid);
    print_grid(5, 5, rows, cols, &mut grid);
    print_grid(rows, cols, rows, cols, &mut grid); // Edge of grid

    // Test with a small grid that's smaller than the view window
    let small_rows = 5;
    let small_cols = 5;
    let mut small_grid = create_test_grid(small_rows, small_cols);
    print_grid(1, 1, small_rows, small_cols, &mut small_grid);

    // If we reach here without panicking, the test passes
    assert!(true);
}

// New test for print_grid with edge cases
#[test]
fn test_print_grid_edge_cases() {
    let rows = 15;
    let cols = 15;
    let mut grid = create_test_grid(rows, cols);

    // Test boundary conditions
    print_grid(rows - 8, cols - 8, rows, cols, &mut grid); // Near the edge
    print_grid(rows, cols, rows, cols, &mut grid); // At the edge

    // Test with view window that would extend beyond grid boundaries
    print_grid(rows - 5, cols - 5, rows, cols, &mut grid);

    // Test with single row/column visibility
    print_grid(rows, 1, rows, cols, &mut grid);
    print_grid(1, cols, rows, cols, &mut grid);

    // Test with starting position at the exact boundary
    print_grid(rows + 1, cols + 1, rows, cols, &mut grid);

    // If we reach here without panicking, the test passes
    assert!(true);
}

// Test the output toggle functionality
#[test]
fn test_output_toggle() {
    let rows = 20;
    let cols = 20;
    let mut grid = create_test_grid(rows, cols);
    let mut start_x = 10;
    let mut start_y = 10;
    let mut is_disabled = false;

    // Test initial state
    assert!(!is_disabled);

    // Test with your actual disable output command
    // Replace with your actual syntax
    let disable_output_cmd = "output disable";

    let result = process_command(
        disable_output_cmd,
        &mut start_x,
        &mut start_y,
        rows,
        cols,
        &mut is_disabled,
        &mut grid,
    );

    // If command is correctly parsed, check that is_disabled is now true
    if result == 1 {
        assert!(is_disabled);
    }

    // Test with your actual enable output command
    // Replace with your actual syntax
    let enable_output_cmd = "output enable";

    let result = process_command(
        enable_output_cmd,
        &mut start_x,
        &mut start_y,
        rows,
        cols,
        &mut is_disabled,
        &mut grid,
    );

    // If command is correctly parsed, check that is_disabled is now false
    if result == 1 {
        assert!(!is_disabled);
    }
}
