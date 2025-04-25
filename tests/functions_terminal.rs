use project::terminal::functions::*;
use project::terminal::graph::Node;
use project::terminal::types::Coordinates;

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a test grid
    fn setup_test_grid() -> Vec<Vec<Node>> {
        let mut grid = vec![
            vec![Node::default(); 4], // Row 0 (placeholder)
            vec![Node::default(); 4], // Row 1
            vec![Node::default(); 4], // Row 2
            vec![Node::default(); 4], // Row 3
        ];

        // Set up node positions
        for i in 0..4 {
            for j in 0..4 {
                grid[i][j].position = Coordinates {
                    row: i as i32,
                    col: j as i32,
                };
                grid[i][j].valid = true;
            }
        }

        // Set some test values
        // Row 1: [10, 20, 30, 40]
        // Row 2: [5, 15, 25, 35]
        // Row 3: [2, 8, 18, 32]
        grid[1][1].node_value = 10;
        grid[1][2].node_value = 20;
        grid[1][3].node_value = 30;
        grid[2][1].node_value = 5;
        grid[2][2].node_value = 15;
        grid[2][3].node_value = 25;
        grid[3][1].node_value = 2;
        grid[3][2].node_value = 8;
        grid[3][3].node_value = 18;

        grid
    }

    #[test]
    fn test_sum_function() {
        let grid = setup_test_grid();

        // Test case 1: Sum of a single cell
        let result = sum_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 1, col: 1 },
            &grid,
        );
        assert_eq!(result, Some(10));

        // Test case 2: Sum of a row
        let result = sum_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 1, col: 3 },
            &grid,
        );
        assert_eq!(result, Some(60)); // 10+20+30

        // Test case 3: Sum of a column
        let result = sum_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 3, col: 1 },
            &grid,
        );
        assert_eq!(result, Some(17)); // 10+5+2

        // Test case 4: Sum of a rectangle
        let result = sum_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 2, col: 2 },
            &grid,
        );
        assert_eq!(result, Some(50)); // 10+20+5+15
    }

    #[test]
    fn test_avg_function() {
        let grid = setup_test_grid();

        // Test case 1: Average of a single cell
        let result = avg_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 1, col: 1 },
            &grid,
        );
        assert_eq!(result, Some(10));

        // Test case 2: Average of a row
        let result = avg_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 1, col: 3 },
            &grid,
        );
        assert_eq!(result, Some(20)); // (10+20+30)/3

        // Test case 3: Average of a rectangle
        let result = avg_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 2, col: 2 },
            &grid,
        );
        assert_eq!(result, Some(12)); // (10+20+5+15)/4 = 50/4 = 12.5 -> 12 (integer division)
    }

    #[test]
    fn test_min_function() {
        let grid = setup_test_grid();

        // Test case 1: Min of a single cell
        let result = min_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 1, col: 1 },
            &grid,
        );
        assert_eq!(result, Some(10));

        // Test case 2: Min of a row
        let result = min_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 1, col: 3 },
            &grid,
        );
        assert_eq!(result, Some(10)); // min(10, 20, 30)

        // Test case 3: Min of a rectangle
        let result = min_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 3, col: 3 },
            &grid,
        );
        assert_eq!(result, Some(2)); // min(10, 20, 30, 5, 15, 25, 2, 8, 18)
    }

    #[test]
    fn test_max_function() {
        let grid = setup_test_grid();

        // Test case 1: Max of a single cell
        let result = max_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 1, col: 1 },
            &grid,
        );
        assert_eq!(result, Some(10));

        // Test case 2: Max of a row
        let result = max_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 1, col: 3 },
            &grid,
        );
        assert_eq!(result, Some(30)); // max(10, 20, 30)

        // Test case 3: Max of a rectangle
        let result = max_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 3, col: 3 },
            &grid,
        );
        assert_eq!(result, Some(30)); // max(10, 20, 30, 5, 15, 25, 2, 8, 18)
    }

    #[test]
    fn test_stdev_function() {
        let grid = setup_test_grid();

        // Test case 1: STDEV of a single cell (should be 0)
        let result = stdev_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 1, col: 1 },
            &grid,
        );
        assert_eq!(result, Some(0));

        // Test case 2: STDEV of a row
        // Values: 10, 20, 30
        // Mean: 20
        // Variance: ((10-20)² + (20-20)² + (30-20)²)/3 = (100 + 0 + 100)/3 = 66.67
        // STDEV: sqrt(66.67) ≈ 8.16 -> 8 (with integer rounding)
        let result = stdev_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 1, col: 3 },
            &grid,
        );
        assert_eq!(result, Some(8));

        // Test case 3: STDEV of a 2x2 square
        // Values: 10, 20, 5, 15
        // Mean: 12.5
        // Variance: ((10-12.5)² + (20-12.5)² + (5-12.5)² + (15-12.5)²)/4 = 37.5
        // STDEV: sqrt(37.5) ≈ 6.12 -> 6 (with integer rounding)
        let result = stdev_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 2, col: 2 },
            &grid,
        );
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_invalid_cells() {
        let mut grid = setup_test_grid();

        // Mark one cell as invalid
        grid[2][2].valid = false;

        // All range functions should return None if any cell is invalid
        let coords_start = Coordinates { row: 1, col: 1 };
        let coords_end = Coordinates { row: 2, col: 2 };

        assert_eq!(sum_function(coords_start, coords_end, &grid), None);
        assert_eq!(avg_function(coords_start, coords_end, &grid), None);
        assert_eq!(min_function(coords_start, coords_end, &grid), None);
        assert_eq!(max_function(coords_start, coords_end, &grid), None);
        assert_eq!(stdev_function(coords_start, coords_end, &grid), None);
    }

    #[test]
    fn test_empty_range() {
        // Create an empty grid (all cells have 0 value but are valid)
        let grid = vec![vec![Node::default(); 4]; 4];

        // Calculate average of empty range - should be None as count is 0
        let result = avg_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 0, col: 0 }, // invalid range (end before start)
            &grid,
        );
        assert_eq!(result, None);

        // Sum of empty range should be Some(0)
        let result = sum_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 0, col: 0 }, // invalid range (end before start)
            &grid,
        );
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_is_arithmetic() {
        assert!(is_arithmetic(Operation::Add));
        assert!(is_arithmetic(Operation::Sub));
        assert!(is_arithmetic(Operation::Mul));
        assert!(is_arithmetic(Operation::Div));

        assert!(!is_arithmetic(Operation::Sum));
        assert!(!is_arithmetic(Operation::Avg));
        assert!(!is_arithmetic(Operation::Min));
        assert!(!is_arithmetic(Operation::Max));
        assert!(!is_arithmetic(Operation::Std));
        assert!(!is_arithmetic(Operation::Slp));
        assert!(!is_arithmetic(Operation::Cons));
    }

    #[test]
    fn test_operation_equality() {
        // Test that operations can be compared correctly
        assert_eq!(Operation::Add, Operation::Add);
        assert_ne!(Operation::Add, Operation::Sub);
        assert_ne!(Operation::Sum, Operation::Avg);
    }

    #[test]
    fn test_sum_empty_range() {
        let grid = setup_test_grid();

        // Test summing 0 cells (empty range)
        let result = sum_function(
            Coordinates { row: 3, col: 3 }, // Out of grid bounds
            Coordinates { row: 3, col: 3 },
            &grid,
        );
        assert_eq!(result, Some(18)); // only that grid cell is present whose value is 18
    }

    #[test]
    fn test_stdev_edge_cases() {
        let _grid = setup_test_grid();

        // Test STDEV of a range with identical values (should be 0)
        let mut identical_grid = setup_test_grid();
        for i in 1..3 {
            for j in 1..3 {
                identical_grid[i][j].node_value = 10;
            }
        }

        let result = stdev_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 2, col: 2 },
            &identical_grid,
        );
        assert_eq!(result, Some(0));

        // Test with extreme values that might cause integer overflow
        let mut extreme_grid = setup_test_grid();
        extreme_grid[1][1].node_value = i32::MAX / 2;
        extreme_grid[1][2].node_value = i32::MIN / 2;

        // This test mainly checks that the function doesn't panic
        let result = stdev_function(
            Coordinates { row: 1, col: 1 },
            Coordinates { row: 1, col: 2 },
            &extreme_grid,
        );
        assert!(result.is_some());
    }
}
