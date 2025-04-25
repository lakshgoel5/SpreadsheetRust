use project::terminal::backend::*;
use project::terminal::functions::Operation;
#[allow(unused_imports)]
use project::terminal::graph::Node;
use project::terminal::types::Coordinates;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_grid() {
        let grid = generate_grid(3, 3);

        // Check grid dimensions
        assert_eq!(grid.len(), 4); // 0-indexed + specified rows
        assert_eq!(grid[0].len(), 4); // 0-indexed + specified columns

        // Check node initialization
        let node = &grid[1][2]; // Random cell
        assert_eq!(node.node_value, 0);
        assert_eq!(node.value1, Coordinates { row: -1, col: -1 });
        assert_eq!(node.value2, Coordinates { row: -1, col: -1 });
        assert_eq!(node.position, Coordinates { row: 1, col: 2 });
        assert_eq!(node.op, Operation::Cons);
        assert!(node.valid);
        assert!(!node.visited);
        assert!(node.dependents.is_empty());
    }

    #[test]
    fn test_add_edges_binary_operation() {
        let mut grid = generate_grid(3, 3);

        // Create a simple dependency: C1 = A1 + B1
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        add_edges(&mut grid, a1, b1, c1, Operation::Add, true);

        // Check that A1 and B1 have C1 as dependent
        assert!(grid[1][1].dependents.contains(&c1));
        assert!(grid[1][2].dependents.contains(&c1));
    }

    #[test]
    fn test_add_edges_range_operation() {
        let mut grid = generate_grid(3, 3);

        // Create a range dependency: D1 = SUM(A1:B2)
        let a1 = Coordinates { row: 1, col: 1 };
        let b2 = Coordinates { row: 2, col: 2 };
        let d1 = Coordinates { row: 1, col: 3 };

        add_edges(&mut grid, a1, b2, d1, Operation::Sum, true);

        // Check that all cells in range have D1 as dependent
        assert!(grid[1][1].dependents.contains(&d1)); // A1
        assert!(grid[1][2].dependents.contains(&d1)); // B1
        assert!(grid[2][1].dependents.contains(&d1)); // A2
        assert!(grid[2][2].dependents.contains(&d1)); // B2
    }

    #[test]
    fn test_add_edges_range_sleep() {
        let mut grid = generate_grid(3, 3);

        // Create a range dependency: D1 = SUM(A1:B2)
        let a1 = Coordinates { row: -1, col: -1 };
        let b2 = Coordinates { row: 2, col: 2 };
        let d1 = Coordinates { row: 1, col: 3 };

        add_edges(&mut grid, a1, b2, d1, Operation::Slp, true);

        // Check that all cells in range have D1 as dependent
        assert!(!grid[1][1].dependents.contains(&d1)); // A1
    }

    #[test]
    fn test_add_edges_range_enable() {
        let mut grid = generate_grid(3, 3);

        // Create a range dependency: D1 = SUM(A1:B2)
        let a1 = Coordinates { row: -1, col: -1 };
        let b2 = Coordinates { row: 2, col: 2 };
        let d1 = Coordinates { row: 1, col: 3 };

        add_edges(&mut grid, a1, b2, d1, Operation::EnableOutput, true);

        // Check that all cells in range have D1 as dependent
        assert!(!grid[1][1].dependents.contains(&d1)); // A1
        assert!(!grid[1][2].dependents.contains(&d1)); // B1
        assert!(!grid[2][1].dependents.contains(&d1)); // A2
        assert!(!grid[2][2].dependents.contains(&d1)); // B2
    }

    #[test]
    fn test_break_edges_binary_operation() {
        let mut grid = generate_grid(3, 3);

        // Setup: C1 = A1 + B1
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // First add edges
        add_edges(&mut grid, a1, b1, c1, Operation::Add, true);

        // Then break them
        break_edges(&mut grid, a1, b1, c1, Operation::Add, true);

        // Check that A1 and B1 no longer have C1 as dependent
        assert!(!grid[1][1].dependents.contains(&c1));
        assert!(!grid[1][2].dependents.contains(&c1));
    }

    #[test]
    fn test_break_edges_range_operation() {
        let mut grid = generate_grid(3, 3);

        // Setup: D1 = SUM(A1:B2)
        let a1 = Coordinates { row: 1, col: 1 };
        let b2 = Coordinates { row: 2, col: 2 };
        let d1 = Coordinates { row: 1, col: 3 };

        // First add edges
        add_edges(&mut grid, a1, b2, d1, Operation::Sum, true);

        // Then break them
        break_edges(&mut grid, a1, b2, d1, Operation::Sum, true);

        // Check that all cells in range no longer have D1 as dependent
        assert!(!grid[1][1].dependents.contains(&d1)); // A1
        assert!(!grid[1][2].dependents.contains(&d1)); // B1
        assert!(!grid[2][1].dependents.contains(&d1)); // A2
        assert!(!grid[2][2].dependents.contains(&d1)); // B2
    }

    #[test]
    fn test_has_cycle_no_cycle() {
        let mut grid = generate_grid(3, 3);

        // Setup: B1 = A1 + 5
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let five = Coordinates { row: 5, col: -1 }; // constant value 5

        add_edges(&mut grid, a1, five, b1, Operation::Add, true);

        // This shouldn't create a cycle
        assert!(!has_cycle(b1, &mut grid));
    }

    #[test]
    fn test_has_cycle_direct_cycle() {
        let mut grid = generate_grid(3, 3);

        // Setup: A1 = B1 + 5, B1 = A1 + 3
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let five = Coordinates { row: 5, col: -1 }; // constant value 5
        let three = Coordinates { row: 3, col: -1 }; // constant value 3

        // First add A1 = B1 + 5
        add_edges(&mut grid, b1, five, a1, Operation::Add, true);

        // Now try to add B1 = A1 + 3 (which creates a cycle)
        add_edges(&mut grid, a1, three, b1, Operation::Add, true);

        // This should detect a cycle
        assert!(has_cycle(b1, &mut grid));
    }

    #[test]
    fn test_has_cycle_indirect_cycle() {
        let mut grid = generate_grid(3, 3);

        // Setup: A1 = C1 + 1, B1 = A1 + 2, C1 = B1 + 3
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };
        let one = Coordinates { row: 1, col: -1 };
        let two = Coordinates { row: 2, col: -1 };
        let three = Coordinates { row: 3, col: -1 };

        // Add first two relationships
        add_edges(&mut grid, c1, one, a1, Operation::Add, true);
        add_edges(&mut grid, a1, two, b1, Operation::Add, true);

        // Now try to add C1 = B1 + 3 (which creates an indirect cycle)
        add_edges(&mut grid, b1, three, c1, Operation::Add, true);

        // This should detect a cycle
        assert!(has_cycle(c1, &mut grid));
    }

    #[test]
    fn test_evaluate_node_constant() {
        let mut grid = generate_grid(3, 3);

        // Set A1 = 42
        let a1 = Coordinates { row: 1, col: 1 };
        let val = Coordinates { row: 42, col: -1 }; // constant value 42

        grid[1][1].op = Operation::Cons;
        grid[1][1].value1 = val;

        evaluate_node(&mut grid, a1);

        // Check that A1 has value 42
        assert_eq!(grid[1][1].node_value, 42);
        assert!(grid[1][1].valid);
    }

    #[test]
    fn test_evaluate_node_binary_add() {
        let mut grid = generate_grid(3, 3);

        // Set A1 = 10, B1 = 20, C1 = A1 + B1
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // Set values for A1 and B1
        grid[1][1].op = Operation::Cons;
        grid[1][1].value1 = Coordinates { row: 10, col: -1 };
        grid[1][2].op = Operation::Cons;
        grid[1][2].value1 = Coordinates { row: 20, col: -1 };

        // Evaluate them first
        evaluate_node(&mut grid, a1);
        evaluate_node(&mut grid, b1);

        // Set C1 = A1 + B1
        grid[1][3].op = Operation::Add;
        grid[1][3].value1 = a1;
        grid[1][3].value2 = b1;

        // Add dependencies
        add_edges(&mut grid, a1, b1, c1, Operation::Add, true);

        // Evaluate C1
        evaluate_node(&mut grid, c1);

        // Check result
        assert_eq!(grid[1][3].node_value, 30); // 10 + 20
        assert!(grid[1][3].valid);
    }

    #[test]
    fn test_evaluate_node_binary_add_1() {
        let mut grid = generate_grid(3, 3);

        // Set A1 = 10, B1 = 20, C1 = A1 + B1
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // Set values for A1 and B1
        grid[1][1].op = Operation::Cons;
        grid[1][1].value1 = Coordinates { row: 10, col: -1 };
        grid[1][2].op = Operation::Cons;
        grid[1][2].value1 = Coordinates { row: 20, col: -1 };

        // Evaluate them first
        evaluate_node(&mut grid, a1);
        evaluate_node(&mut grid, b1);

        // Set C1 = A1 + B1
        grid[1][3].op = Operation::Add;
        grid[1][3].value1 = Coordinates { row: 1, col: -1 };
        grid[1][3].value2 = Coordinates { row: 1, col: 2 };

        // Add dependencies
        add_edges(&mut grid, a1, b1, c1, Operation::Add, true);

        // Evaluate C1
        evaluate_node(&mut grid, c1);

        // Check result
        assert_eq!(grid[1][3].node_value, 21); // 10 + 20
        assert!(grid[1][3].valid);
    }

    #[test]
    fn test_evaluate_node_binary_add_2() {
        let mut grid = generate_grid(3, 3);

        // Set A1 = 10, B1 = 20, C1 = A1 + B1
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // Set values for A1 and B1
        grid[1][1].op = Operation::Cons;
        grid[1][1].value1 = Coordinates { row: 10, col: -1 };
        grid[1][2].op = Operation::Cons;
        grid[1][2].value1 = Coordinates { row: 20, col: -1 };

        // Evaluate them first
        evaluate_node(&mut grid, a1);
        evaluate_node(&mut grid, b1);

        // Set C1 = A1 + B1
        grid[1][3].op = Operation::Add;
        grid[1][3].value1 = Coordinates { row: 1, col: 1 };
        grid[1][3].value2 = Coordinates { row: 1, col: -1 };

        // Add dependencies
        add_edges(&mut grid, a1, b1, c1, Operation::Add, true);

        // Evaluate C1
        evaluate_node(&mut grid, c1);

        // Check result
        assert_eq!(grid[1][3].node_value, 11); // 10 + 20
        assert!(grid[1][3].valid);
    }

    #[test]
    fn test_evaluate_node_binary_add_3() {
        let mut grid = generate_grid(3, 3);

        // Set A1 = 10, B1 = 20, C1 = A1 + B1
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // Set values for A1 and B1
        grid[1][1].op = Operation::Cons;
        grid[1][1].value1 = Coordinates { row: 10, col: -1 };
        grid[1][2].op = Operation::Cons;
        grid[1][2].value1 = Coordinates { row: 20, col: -1 };

        // Evaluate them first
        evaluate_node(&mut grid, a1);
        evaluate_node(&mut grid, b1);

        // Set C1 = A1 + B1
        grid[1][3].op = Operation::Add;
        grid[1][3].value1 = Coordinates { row: 1, col: -1 };
        grid[1][3].value2 = Coordinates { row: 1, col: -1 };

        // Add dependencies
        add_edges(&mut grid, a1, b1, c1, Operation::Add, true);

        // Evaluate C1
        evaluate_node(&mut grid, c1);

        // Check result
        assert_eq!(grid[1][3].node_value, 2); // 10 + 20
        assert!(grid[1][3].valid);
    }

    #[test]
    fn test_evaluate_node_binary_sub() {
        let mut grid = generate_grid(3, 3);

        // Set A1 = 30, B1 = 12, C1 = A1 - B1
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // Set values for A1 and B1
        grid[1][1].op = Operation::Cons;
        grid[1][1].value1 = Coordinates { row: 30, col: -1 };
        grid[1][2].op = Operation::Cons;
        grid[1][2].value1 = Coordinates { row: 12, col: -1 };

        // Evaluate them first
        evaluate_node(&mut grid, a1);
        evaluate_node(&mut grid, b1);

        // Set C1 = A1 - B1
        grid[1][3].op = Operation::Sub;
        grid[1][3].value1 = a1;
        grid[1][3].value2 = b1;

        // Add dependencies
        add_edges(&mut grid, a1, b1, c1, Operation::Sub, true);

        // Evaluate C1
        evaluate_node(&mut grid, c1);

        // Check result
        assert_eq!(grid[1][3].node_value, 18); // 30 - 12
        assert!(grid[1][3].valid);
    }

    #[test]
    fn test_evaluate_node_binary_sub1() {
        let mut grid = generate_grid(3, 3);

        // Set A1 = 10, B1 = 20, C1 = A1 + B1
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // Set values for A1 and B1
        grid[1][1].op = Operation::Cons;
        grid[1][1].value1 = Coordinates { row: 10, col: -1 };
        grid[1][2].op = Operation::Cons;
        grid[1][2].value1 = Coordinates { row: 20, col: -1 };

        // Evaluate them first
        evaluate_node(&mut grid, a1);
        evaluate_node(&mut grid, b1);

        // Set C1 = A1 + B1
        grid[1][3].op = Operation::Sub;
        grid[1][3].value1 = Coordinates { row: 1, col: -1 };
        grid[1][3].value2 = Coordinates { row: 1, col: 2 };

        // Add dependencies
        add_edges(&mut grid, a1, b1, c1, Operation::Sub, true);

        // Evaluate C1
        evaluate_node(&mut grid, c1);

        // Check result
        assert_eq!(grid[1][3].node_value, -19); // 10 + 20
        assert!(grid[1][3].valid);
    }

    #[test]
    fn test_evaluate_node_binary_sub2() {
        let mut grid = generate_grid(3, 3);

        // Set A1 = 10, B1 = 20, C1 = A1 + B1
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // Set values for A1 and B1
        grid[1][1].op = Operation::Cons;
        grid[1][1].value1 = Coordinates { row: 10, col: -1 };
        grid[1][2].op = Operation::Cons;
        grid[1][2].value1 = Coordinates { row: 20, col: -1 };

        // Evaluate them first
        evaluate_node(&mut grid, a1);
        evaluate_node(&mut grid, b1);

        // Set C1 = A1 + B1
        grid[1][3].op = Operation::Sub;
        grid[1][3].value1 = Coordinates { row: 1, col: 1 };
        grid[1][3].value2 = Coordinates { row: 1, col: -1 };

        // Add dependencies
        add_edges(&mut grid, a1, b1, c1, Operation::Sub, true);

        // Evaluate C1
        evaluate_node(&mut grid, c1);

        // Check result
        assert_eq!(grid[1][3].node_value, 9); // 10 + 20
        assert!(grid[1][3].valid);
    }

    #[test]
    fn test_evaluate_node_binary_sub3() {
        let mut grid = generate_grid(3, 3);

        // Set A1 = 10, B1 = 20, C1 = A1 + B1
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // Set values for A1 and B1
        grid[1][1].op = Operation::Cons;
        grid[1][1].value1 = Coordinates { row: 10, col: -1 };
        grid[1][2].op = Operation::Cons;
        grid[1][2].value1 = Coordinates { row: 20, col: -1 };

        // Evaluate them first
        evaluate_node(&mut grid, a1);
        evaluate_node(&mut grid, b1);

        // Set C1 = A1 + B1
        grid[1][3].op = Operation::Sub;
        grid[1][3].value1 = Coordinates { row: 1, col: -1 };
        grid[1][3].value2 = Coordinates { row: 1, col: -1 };

        // Add dependencies
        add_edges(&mut grid, a1, b1, c1, Operation::Sub, true);

        // Evaluate C1
        evaluate_node(&mut grid, c1);

        // Check result
        assert_eq!(grid[1][3].node_value, 0); // 10 + 20
        assert!(grid[1][3].valid);
    }

    #[test]
    fn test_evaluate_node_binary_mul() {
        let mut grid = generate_grid(3, 3);

        // Set A1 = 7, B1 = 6, C1 = A1 * B1
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // Set values for A1 and B1
        grid[1][1].op = Operation::Cons;
        grid[1][1].value1 = Coordinates { row: 7, col: -1 };
        grid[1][2].op = Operation::Cons;
        grid[1][2].value1 = Coordinates { row: 6, col: -1 };

        // Evaluate them first
        evaluate_node(&mut grid, a1);
        evaluate_node(&mut grid, b1);

        // Set C1 = A1 * B1
        grid[1][3].op = Operation::Mul;
        grid[1][3].value1 = a1;
        grid[1][3].value2 = b1;

        // Add dependencies
        add_edges(&mut grid, a1, b1, c1, Operation::Mul, true);

        // Evaluate C1
        evaluate_node(&mut grid, c1);

        // Check result
        assert_eq!(grid[1][3].node_value, 42); // 7 * 6
        assert!(grid[1][3].valid);
    }

    #[test]
    fn test_evaluate_node_binary_mul1() {
        let mut grid = generate_grid(3, 3);

        // Set A1 = 10, B1 = 20, C1 = A1 + B1
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // Set values for A1 and B1
        grid[1][1].op = Operation::Cons;
        grid[1][1].value1 = Coordinates { row: 10, col: -1 };
        grid[1][2].op = Operation::Cons;
        grid[1][2].value1 = Coordinates { row: 20, col: -1 };

        // Evaluate them first
        evaluate_node(&mut grid, a1);
        evaluate_node(&mut grid, b1);

        // Set C1 = A1 + B1
        grid[1][3].op = Operation::Mul;
        grid[1][3].value1 = Coordinates { row: 1, col: -1 };
        grid[1][3].value2 = Coordinates { row: 1, col: 2 };

        // Add dependencies
        add_edges(&mut grid, a1, b1, c1, Operation::Mul, true);

        // Evaluate C1
        evaluate_node(&mut grid, c1);

        // Check result
        assert_eq!(grid[1][3].node_value, 20); // 10 + 20
        assert!(grid[1][3].valid);
    }

    #[test]
    fn test_evaluate_node_binary_mul2() {
        let mut grid = generate_grid(3, 3);

        // Set A1 = 10, B1 = 20, C1 = A1 + B1
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // Set values for A1 and B1
        grid[1][1].op = Operation::Cons;
        grid[1][1].value1 = Coordinates { row: 10, col: -1 };
        grid[1][2].op = Operation::Cons;
        grid[1][2].value1 = Coordinates { row: 20, col: -1 };

        // Evaluate them first
        evaluate_node(&mut grid, a1);
        evaluate_node(&mut grid, b1);

        // Set C1 = A1 + B1
        grid[1][3].op = Operation::Mul;
        grid[1][3].value1 = Coordinates { row: 1, col: 1 };
        grid[1][3].value2 = Coordinates { row: 1, col: -1 };

        // Add dependencies
        add_edges(&mut grid, a1, b1, c1, Operation::Mul, true);

        // Evaluate C1
        evaluate_node(&mut grid, c1);

        // Check result
        assert_eq!(grid[1][3].node_value, 10); // 10 + 20
        assert!(grid[1][3].valid);
    }

    #[test]
    fn test_evaluate_node_binary_mul3() {
        let mut grid = generate_grid(3, 3);

        // Set A1 = 10, B1 = 20, C1 = A1 + B1
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // Set values for A1 and B1
        grid[1][1].op = Operation::Cons;
        grid[1][1].value1 = Coordinates { row: 10, col: -1 };
        grid[1][2].op = Operation::Cons;
        grid[1][2].value1 = Coordinates { row: 20, col: -1 };

        // Evaluate them first
        evaluate_node(&mut grid, a1);
        evaluate_node(&mut grid, b1);

        // Set C1 = A1 + B1
        grid[1][3].op = Operation::Mul;
        grid[1][3].value1 = Coordinates { row: 1, col: -1 };
        grid[1][3].value2 = Coordinates { row: 1, col: -1 };

        // Add dependencies
        add_edges(&mut grid, a1, b1, c1, Operation::Mul, true);

        // Evaluate C1
        evaluate_node(&mut grid, c1);

        // Check result
        assert_eq!(grid[1][3].node_value, 1); // 10 + 20
        assert!(grid[1][3].valid);
    }
    #[test]
    fn test_evaluate_node_binary_div() {
        let mut grid = generate_grid(3, 3);

        // Set A1 = 20, B1 = 4, C1 = A1 / B1
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // Set values for A1 and B1
        grid[1][1].op = Operation::Cons;
        grid[1][1].value1 = Coordinates { row: 20, col: -1 };
        grid[1][2].op = Operation::Cons;
        grid[1][2].value1 = Coordinates { row: 4, col: -1 };

        // Evaluate them first
        evaluate_node(&mut grid, a1);
        evaluate_node(&mut grid, b1);

        // Set C1 = A1 / B1
        grid[1][3].op = Operation::Div;
        grid[1][3].value1 = a1;
        grid[1][3].value2 = b1;

        // Add dependencies
        add_edges(&mut grid, a1, b1, c1, Operation::Div, true);

        // Evaluate C1
        evaluate_node(&mut grid, c1);

        // Check result
        assert_eq!(grid[1][3].node_value, 5); // 20 / 4
        assert!(grid[1][3].valid);
    }

    #[test]
    fn test_evaluate_node_div_by_zero() {
        let mut grid = generate_grid(3, 3);

        // Set A1 = 20, B1 = 0, C1 = A1 / B1
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // Set values for A1 and B1
        grid[1][1].op = Operation::Cons;
        grid[1][1].value1 = Coordinates { row: 20, col: -1 };
        grid[1][2].op = Operation::Cons;
        grid[1][2].value1 = Coordinates { row: 0, col: -1 };

        // Evaluate them first
        evaluate_node(&mut grid, a1);
        evaluate_node(&mut grid, b1);

        // Set C1 = A1 / B1
        grid[1][3].op = Operation::Div;
        grid[1][3].value1 = a1;
        grid[1][3].value2 = b1;

        // Add dependencies
        add_edges(&mut grid, a1, b1, c1, Operation::Div, true);

        // Evaluate C1
        evaluate_node(&mut grid, c1);

        // Check result - should be invalid due to division by zero
        assert!(!grid[1][3].valid);
    }

    #[test]
    fn test_getting_things_updated_success() {
        let mut grid = generate_grid(3, 3);

        // Set A1 = 10, B1 = 20
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // Set up initial values
        grid[1][1].op = Operation::Cons;
        grid[1][1].value1 = Coordinates { row: 10, col: -1 };
        grid[1][2].op = Operation::Cons;
        grid[1][2].value1 = Coordinates { row: 20, col: -1 };

        // Evaluate initial values
        evaluate_node(&mut grid, a1);
        evaluate_node(&mut grid, b1);

        // Now update C1 = A1 + B1
        let result = getting_things_updated(&mut grid, c1, a1, b1, Operation::Add);

        // Check result code and cell value
        assert_eq!(result, 1); // Success
        assert_eq!(grid[1][3].node_value, 30); // 10 + 20
    }

    #[test]
    fn test_getting_things_updated_cycle_detection() {
        let mut grid = generate_grid(3, 3);

        // Set up A1 = 10, B1 = A1 * 2
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let two = Coordinates { row: 2, col: -1 };

        // Set initial value for A1 = 10
        grid[1][1].op = Operation::Cons;
        grid[1][1].value1 = Coordinates { row: 10, col: -1 };
        evaluate_node(&mut grid, a1);

        // Set B1 = A1 * 2
        grid[1][2].op = Operation::Mul;
        grid[1][2].value1 = a1;
        grid[1][2].value2 = two;
        add_edges(&mut grid, a1, two, b1, Operation::Mul, true);
        evaluate_node(&mut grid, b1);

        // Now try to update A1 = B1 + 5 (which would create a cycle)
        let five = Coordinates { row: 5, col: -1 };
        let result = getting_things_updated(&mut grid, a1, b1, five, Operation::Add);

        // Should detect cycle and return error code
        assert_eq!(result, 5); // Cycle detected

        // A1 should retain its original value
        assert_eq!(grid[1][1].node_value, 10);
    }

    #[test]
    fn test_update_topo_simple_chain() {
        let mut grid = generate_grid(3, 3);

        // Set A1 = 10, B1 = A1 * 2, C1 = B1 + 5
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };
        let two = Coordinates { row: 2, col: -1 };
        let five = Coordinates { row: 5, col: -1 };

        // Set up initial values
        grid[1][1].op = Operation::Cons;
        grid[1][1].value1 = Coordinates { row: 10, col: -1 };
        evaluate_node(&mut grid, a1);

        // Set B1 = A1 * 2
        grid[1][2].op = Operation::Mul;
        grid[1][2].value1 = a1;
        grid[1][2].value2 = two;
        add_edges(&mut grid, a1, two, b1, Operation::Mul, true);

        // Set C1 = B1 + 5
        grid[1][3].op = Operation::Add;
        grid[1][3].value1 = b1;
        grid[1][3].value2 = five;
        add_edges(&mut grid, b1, five, c1, Operation::Add, true);

        // Update starting from A1
        update_topo(&mut grid, a1);

        // Check the propagated values
        assert_eq!(grid[1][1].node_value, 10); // A1 = 10
        assert_eq!(grid[1][2].node_value, 20); // B1 = A1 * 2 = 10 * 2 = 20
        assert_eq!(grid[1][3].node_value, 25); // C1 = B1 + 5 = 20 + 5 = 25
    }

    #[test]
    fn test_reset_visited() {
        let mut grid = generate_grid(3, 3);

        // Set up a simple structure with dependents
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // Mark all as visited
        grid[1][1].visited = true;
        grid[1][2].visited = true;
        grid[1][3].visited = true;

        // Set up dependencies: A1 -> B1 -> C1
        grid[1][1].dependents.push(b1);
        grid[1][2].dependents.push(c1);

        // Reset visited flags starting from A1
        reset_visited(&mut grid, a1);

        // Check that all flags were reset
        assert!(!grid[1][1].visited);
        assert!(!grid[1][2].visited);
        assert!(!grid[1][3].visited);
    }

    #[test]
    fn test_topological_sort() {
        let mut grid = generate_grid(3, 3);

        // Set up a simple structure with dependencies: A1 -> B1 -> C1
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // Set up dependencies
        grid[1][1].dependents.push(b1);
        grid[1][2].dependents.push(c1);

        // Perform topological sort
        let mut stack = Vec::new();
        topological_sort(&mut grid, a1, &mut stack);

        // Check the order in the stack (it should be C1, B1, A1)
        assert_eq!(stack.len(), 3);
        assert_eq!(stack[0], c1); // C1 should be first out of the stack
        assert_eq!(stack[1], b1); // B1 should be second
        assert_eq!(stack[2], a1); // A1 should be last
    }
    #[test]
    fn test_evaluate_node_sleep() {
        let mut grid = generate_grid(3, 3);
        let a1 = Coordinates { row: 1, col: 1 };

        grid[1][1].op = Operation::Slp;
        grid[1][1].value1 = Coordinates { row: 1, col: -1 };

        evaluate_node(&mut grid, a1);

        assert_eq!(grid[1][1].node_value, 1);
        assert!(grid[1][1].valid);
    }
    #[test]
    fn test_evaluate_node_std_range() {
        let mut grid = generate_grid(3, 3);
        let a1 = Coordinates { row: 1, col: 1 };
        let b2 = Coordinates { row: 2, col: 2 };
        let d1 = Coordinates { row: 1, col: 3 };

        // Fill A1:B2 with constants
        for i in 1..=2 {
            for j in 1..=2 {
                grid[i][j].op = Operation::Cons;
                grid[i][j].value1 = Coordinates {
                    row: 5 * (i + j) as i32,
                    col: -1,
                };
                evaluate_node(
                    &mut grid,
                    Coordinates {
                        row: i as i32,
                        col: j as i32,
                    },
                );
            }
        }

        grid[1][3].op = Operation::Std;
        grid[1][3].value1 = a1;
        grid[1][3].value2 = b2;

        evaluate_node(&mut grid, d1);

        assert!(grid[1][3].valid);
    }
    #[test]
    fn test_evaluate_node_min_range() {
        let mut grid = generate_grid(3, 3);
        let a1 = Coordinates { row: 1, col: 1 };
        let b2 = Coordinates { row: 2, col: 2 };
        let d1 = Coordinates { row: 1, col: 3 };

        // Fill A1:B2 with constants
        for i in 1..=2 {
            for j in 1..=2 {
                grid[i][j].op = Operation::Cons;
                grid[i][j].value1 = Coordinates {
                    row: 5 * (i + j) as i32,
                    col: 3,
                };
                evaluate_node(
                    &mut grid,
                    Coordinates {
                        row: i as i32,
                        col: j as i32,
                    },
                );
            }
        }

        grid[1][3].op = Operation::Min;
        grid[1][3].value1 = a1;
        grid[1][3].value2 = b2;

        evaluate_node(&mut grid, d1);

        assert!(grid[1][3].valid);
    }
    #[test]
    fn test_evaluate_node_max_range() {
        let mut grid = generate_grid(3, 3);
        let a1 = Coordinates { row: 1, col: 1 };
        let b2 = Coordinates { row: 2, col: 2 };
        let d1 = Coordinates { row: 1, col: 3 };

        // Fill A1:B2 with constants
        for i in 1..=2 {
            for j in 1..=2 {
                grid[i][j].op = Operation::Cons;
                grid[i][j].value1 = Coordinates {
                    row: 5 * (i + j) as i32,
                    col: -1,
                };
                evaluate_node(
                    &mut grid,
                    Coordinates {
                        row: i as i32,
                        col: j as i32,
                    },
                );
            }
        }

        grid[1][3].op = Operation::Max;
        grid[1][3].value1 = a1;
        grid[1][3].value2 = b2;

        evaluate_node(&mut grid, d1);

        assert!(grid[1][3].valid);
    }
    #[test]
    fn test_evaluate_node_avg_range() {
        let mut grid = generate_grid(3, 3);
        let a1 = Coordinates { row: 1, col: 1 };
        let b2 = Coordinates { row: 2, col: 2 };
        let d1 = Coordinates { row: 1, col: 3 };

        // Fill A1:B2 with constants
        for i in 1..=2 {
            for j in 1..=2 {
                grid[i][j].op = Operation::Cons;
                grid[i][j].value1 = Coordinates {
                    row: 5 * (i + j) as i32,
                    col: -1,
                };
                evaluate_node(
                    &mut grid,
                    Coordinates {
                        row: i as i32,
                        col: j as i32,
                    },
                );
            }
        }

        grid[1][3].op = Operation::Avg;
        grid[1][3].value1 = a1;
        grid[1][3].value2 = b2;

        evaluate_node(&mut grid, d1);

        assert!(grid[1][3].valid);
    }
    #[test]
    fn test_evaluate_node_sum_range() {
        let mut grid = generate_grid(3, 3);
        let a1 = Coordinates { row: 1, col: 1 };
        let b2 = Coordinates { row: 2, col: 2 };
        let d1 = Coordinates { row: 1, col: 3 };

        // Fill A1:B2 with constants
        for i in 1..=2 {
            for j in 1..=2 {
                grid[i][j].op = Operation::Cons;
                grid[i][j].value1 = Coordinates {
                    row: 5 * (i + j) as i32,
                    col: -1,
                };
                evaluate_node(
                    &mut grid,
                    Coordinates {
                        row: i as i32,
                        col: j as i32,
                    },
                );
            }
        }

        grid[1][3].op = Operation::Sum;
        grid[1][3].value1 = a1;
        grid[1][3].value2 = b2;

        evaluate_node(&mut grid, d1);

        assert!(grid[1][3].valid);
    }
    #[test]
    fn test_add_and_break_edges_old_dependencies() {
        let mut grid = generate_grid(3, 3);
        let a1 = Coordinates { row: 1, col: 1 };
        let b1 = Coordinates { row: 1, col: 2 };
        let c1 = Coordinates { row: 1, col: 3 };

        // Initial: C1 = A1 + B1
        getting_things_updated(&mut grid, c1, a1, b1, Operation::Add);

        // Update C1 = A1 - B1 (this should break previous edges using old values)
        getting_things_updated(&mut grid, c1, a1, b1, Operation::Sub);

        // C1 should still be dependent on A1 and B1 (new op)
        assert!(grid[1][1].dependents.contains(&c1));
        assert!(grid[1][2].dependents.contains(&c1));
    }
}
