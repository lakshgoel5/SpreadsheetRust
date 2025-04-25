use project::extension::backend::backend::*;
use project::extension::backend::graph::*;
use project::extension::common::{Operation, Value};

#[cfg(test)]
mod node_tests {
    use super::*;

    // Helper function to create a test grid with predefined size
    fn create_test_grid(rows: usize, cols: usize) -> Grid {
        let mut grid = Grid::new(rows + 1, cols + 1);
        // Initialize with default nodes
        for r in 1..=rows {
            for c in 1..=cols {
                grid.set_node(r, c, Node::new(0));
            }
        }
        grid
    }

    #[test]
    fn test_node_new() {
        let node = Node::new(42);
        assert_eq!(node.get_node_value(), Some(42));
        assert!(node.dependents.is_empty());
        assert_eq!(node.function, None);
        assert!(!node.visited);
        assert!(node.valid);
    }

    #[test]
    fn test_get_node_value() {
        // Test valid node
        let mut node = Node::new(42);
        assert_eq!(node.get_node_value(), Some(42));

        // Test invalid node
        node.valid = false;
        assert_eq!(node.get_node_value(), None);
    }

    #[test]
    fn test_remove_dep() {
        let mut node = Node::new(42);
        let cell1 = Value::Cell(1, 1);
        let cell2 = Value::Cell(2, 2);

        // Add dependencies
        node.dependents.push(cell1.clone());
        node.dependents.push(cell2.clone());
        assert_eq!(node.dependents.len(), 2);

        // Remove one dependency
        node.remove_dep(cell1);
        assert_eq!(node.dependents.len(), 1);
        assert_eq!(node.dependents[0], cell2);

        // Try to remove non-existent dependency (should do nothing)
        node.remove_dep(Value::Cell(3, 3));
        assert_eq!(node.dependents.len(), 1);
    }

    #[test]
    fn test_add_dep() {
        let mut node = Node::new(42);
        let cell1 = Value::Cell(1, 1);

        // Add dependency
        node.add_dep(cell1.clone());
        assert_eq!(node.dependents.len(), 1);
        assert_eq!(node.dependents[0], cell1);

        // Try to add same dependency again (should not duplicate)
        node.add_dep(cell1.clone());
        assert_eq!(node.dependents.len(), 1);

        // Add another dependency
        let cell2 = Value::Cell(2, 2);
        node.add_dep(cell2.clone());
        assert_eq!(node.dependents.len(), 2);
        assert!(node.dependents.contains(&cell1));
        assert!(node.dependents.contains(&cell2));
    }

    #[test]
    fn test_set_dependents() {
        let mut node = Node::new(42);

        // Initial state
        assert!(node.dependents.is_empty());

        // Set dependencies
        let deps = vec![Value::Cell(1, 1), Value::Cell(2, 2)];
        node.set_dependents(deps.clone());
        assert_eq!(node.dependents, deps);

        // Set to empty
        node.set_dependents(vec![]);
        assert!(node.dependents.is_empty());
    }

    #[test]
    fn test_break_edges_sum_operation() {
        let mut grid = create_test_grid(5, 5);

        // Set up a SUM operation: C3 = SUM(A1:B2)
        let target = Value::Cell(3, 3);
        let func = Some(Value::Oper(
            Some(Box::new(Value::Cell(1, 1))),
            Some(Box::new(Value::Cell(2, 2))),
            Operation::Sum,
        ));

        // Add dependencies first
        add_edges(&mut grid, target.clone(), func.clone(), true);

        // Verify dependencies were added
        assert!(grid.get_node(1, 1).dependents.contains(&target));
        assert!(grid.get_node(1, 2).dependents.contains(&target));
        assert!(grid.get_node(2, 1).dependents.contains(&target));
        assert!(grid.get_node(2, 2).dependents.contains(&target));

        // Break the edges
        break_edges(&mut grid, target.clone(), func.clone(), false);

        // Verify dependencies were removed
        assert!(!grid.get_node(1, 1).dependents.contains(&target));
        assert!(!grid.get_node(1, 2).dependents.contains(&target));
        assert!(!grid.get_node(2, 1).dependents.contains(&target));
        assert!(!grid.get_node(2, 2).dependents.contains(&target));
    }

    #[test]
    fn test_break_edges_binary_operation() {
        let mut grid = create_test_grid(5, 5);

        // Set up a binary operation: C3 = A1 + B2
        let target = Value::Cell(3, 3);
        let func = Some(Value::Oper(
            Some(Box::new(Value::Cell(1, 1))),
            Some(Box::new(Value::Cell(2, 2))),
            Operation::Add,
        ));

        // Add dependencies first
        add_edges(&mut grid, target.clone(), func.clone(), true);

        // Verify dependencies were added
        assert!(grid.get_node(1, 1).dependents.contains(&target));
        assert!(grid.get_node(2, 2).dependents.contains(&target));

        // Break the edges
        break_edges(&mut grid, target.clone(), func.clone(), false);

        // Verify dependencies were removed
        assert!(!grid.get_node(1, 1).dependents.contains(&target));
        assert!(!grid.get_node(2, 2).dependents.contains(&target));
    }

    #[test]
    fn test_add_edges_sum_operation() {
        let mut grid = create_test_grid(5, 5);

        // Set up a SUM operation: C3 = SUM(A1:B2)
        let target = Value::Cell(3, 3);
        let func = Some(Value::Oper(
            Some(Box::new(Value::Cell(1, 1))),
            Some(Box::new(Value::Cell(2, 2))),
            Operation::Sum,
        ));

        // Add the edges
        add_edges(&mut grid, target.clone(), func.clone(), true);

        // Verify all cells in the range have the target as a dependent
        assert!(grid.get_node(1, 1).dependents.contains(&target));
        assert!(grid.get_node(1, 2).dependents.contains(&target));
        assert!(grid.get_node(2, 1).dependents.contains(&target));
        assert!(grid.get_node(2, 2).dependents.contains(&target));
    }

    #[test]
    fn test_add_edges_binary_operation() {
        let mut grid = create_test_grid(5, 5);

        // Set up a binary operation: C3 = A1 + B2
        let target = Value::Cell(3, 3);
        let func = Some(Value::Oper(
            Some(Box::new(Value::Cell(1, 1))),
            Some(Box::new(Value::Cell(2, 2))),
            Operation::Add,
        ));

        // Add the edges
        add_edges(&mut grid, target.clone(), func.clone(), true);

        // Verify both operand cells have the target as a dependent
        assert!(grid.get_node(1, 1).dependents.contains(&target));
        assert!(grid.get_node(2, 2).dependents.contains(&target));
    }

    #[test]
    fn test_update_edges() {
        let mut grid = create_test_grid(5, 5);

        // Set up an initial operation: C3 = A1 + B2
        let target = Value::Cell(3, 3);
        let func1 = Some(Value::Oper(
            Some(Box::new(Value::Cell(1, 1))),
            Some(Box::new(Value::Cell(2, 2))),
            Operation::Add,
        ));

        // Store the function in the grid
        grid.get_node(3, 3).function = func1.clone();

        // Add initial dependencies
        update_edges(&mut grid, target.clone(), func1.clone(), true);

        // Verify initial dependencies
        assert!(grid.get_node(1, 1).dependents.contains(&target));
        assert!(grid.get_node(2, 2).dependents.contains(&target));

        // Change the operation: C3 = D4 * E5
        let func2 = Some(Value::Oper(
            Some(Box::new(Value::Cell(4, 4))),
            Some(Box::new(Value::Cell(5, 5))),
            Operation::Mul,
        ));

        // Update the edges with new function
        update_edges(&mut grid, target.clone(), func2.clone(), true);

        // Verify old dependencies are removed
        assert!(!grid.get_node(1, 1).dependents.contains(&target));
        assert!(!grid.get_node(2, 2).dependents.contains(&target));

        // Verify new dependencies are added
        assert!(grid.get_node(4, 4).dependents.contains(&target));
        assert!(grid.get_node(5, 5).dependents.contains(&target));
    }

    #[test]
    fn test_has_cycle_no_cycle() {
        let mut grid = create_test_grid(5, 5);

        // Set up a dependency chain: A1 -> B2 -> C3
        let a1 = Value::Cell(1, 1);
        let b2 = Value::Cell(2, 2);
        let c3 = Value::Cell(3, 3);

        grid.get_node(1, 1).add_dep(b2.clone());
        grid.get_node(2, 2).add_dep(c3.clone());

        // There should be no cycle
        assert!(!has_cycle(&mut grid, a1.clone()));
        assert!(!has_cycle(&mut grid, b2.clone()));
        assert!(!has_cycle(&mut grid, c3.clone()));

        // Verify visited flags were reset
        assert!(!grid.get_node(1, 1).visited);
        assert!(!grid.get_node(2, 2).visited);
        assert!(!grid.get_node(3, 3).visited);
    }

    #[test]
    fn test_has_cycle_with_cycle() {
        let mut grid = create_test_grid(5, 5);

        // Set up a cyclic dependency: A1 -> B2 -> C3 -> A1
        let a1 = Value::Cell(1, 1);
        let b2 = Value::Cell(2, 2);
        let c3 = Value::Cell(3, 3);

        grid.get_node(1, 1).add_dep(b2.clone());
        grid.get_node(2, 2).add_dep(c3.clone());
        grid.get_node(3, 3).add_dep(a1.clone());

        // There should be a cycle detected
        assert!(has_cycle(&mut grid, a1.clone()));

        // Verify visited flags were reset
        assert!(!grid.get_node(1, 1).visited);
        assert!(!grid.get_node(2, 2).visited);
        assert!(!grid.get_node(3, 3).visited);
    }

    #[test]
    fn test_reset_visited() {
        let mut grid = create_test_grid(5, 5);

        // Set up visited flags
        grid.get_node(1, 1).visited = true;
        grid.get_node(2, 2).visited = true;
        grid.get_node(3, 3).visited = true;

        // Set up a dependency chain: A1 -> B2 -> C3
        let a1 = Value::Cell(1, 1);
        let b2 = Value::Cell(2, 2);
        let c3 = Value::Cell(3, 3);

        grid.get_node(1, 1).add_dep(b2.clone());
        grid.get_node(2, 2).add_dep(c3.clone());

        // Reset visited flags
        reset_visited(&mut grid, a1.clone());

        // Verify visited flags were reset
        assert!(!grid.get_node(1, 1).visited);
        assert!(!grid.get_node(2, 2).visited);
        assert!(!grid.get_node(3, 3).visited);
    }

    #[test]
    fn test_topological_sort() {
        let mut grid = create_test_grid(5, 5);

        // Set up a dependency chain: A1 -> B2 -> C3
        let a1 = Value::Cell(1, 1);
        let b2 = Value::Cell(2, 2);
        let c3 = Value::Cell(3, 3);

        grid.get_node(1, 1).add_dep(b2.clone());
        grid.get_node(2, 2).add_dep(c3.clone());

        let mut stack = Vec::new();
        topological_sort(&mut grid, a1.clone(), &mut stack);

        // The stack should contain a valid topological sort (reverse order of DFS finishing times)
        // Expected order: [A1, B2, C3]
        assert_eq!(stack.len(), 3);
        assert_eq!(stack[2], a1);
        assert_eq!(stack[1], b2);
        assert_eq!(stack[0], c3);
    }

    #[test]
    fn test_get_sequence() {
        let mut grid = create_test_grid(5, 5);

        // Set up a dependency graph: A1 -> B2 -> D4, A1 -> C3
        let a1 = Value::Cell(1, 1);
        let b2 = Value::Cell(2, 2);
        let c3 = Value::Cell(3, 3);
        let d4 = Value::Cell(4, 4);

        grid.get_node(1, 1).add_dep(b2.clone());
        grid.get_node(1, 1).add_dep(c3.clone());
        grid.get_node(2, 2).add_dep(d4.clone());

        // Get the topological sequence starting from A1
        let sequence = get_sequence(&mut grid, a1.clone());

        // The sequence should be in evaluation order (opposite of topological sort output)
        // One valid order would be: [C3, D4, B2, A1]
        assert_eq!(sequence.len(), 4);

        // Verify A1 is last in the sequence (must be evaluated last)
        assert_eq!(sequence[0], a1);

        // B2 must come after D4
        let b2_pos = sequence.iter().position(|v| v == &b2).unwrap();
        let d4_pos = sequence.iter().position(|v| v == &d4).unwrap();
        assert!(b2_pos < d4_pos);

        // Verify visited flags were reset
        assert!(!grid.get_node(1, 1).visited);
        assert!(!grid.get_node(2, 2).visited);
        assert!(!grid.get_node(3, 3).visited);
        assert!(!grid.get_node(4, 4).visited);
    }

    #[test]
    fn test_integration_dependency_management() {
        let mut grid = create_test_grid(5, 5);

        // Test scenario: C3 initially depends on A1 and B2, then we change it to depend on D4
        let a1 = Value::Cell(1, 1);
        let b2 = Value::Cell(2, 2);
        let c3 = Value::Cell(3, 3);
        let d4 = Value::Cell(4, 4);

        // Initial formula: C3 = A1 + B2
        let func1 = Some(Value::Oper(
            Some(Box::new(a1.clone())),
            Some(Box::new(b2.clone())),
            Operation::Add,
        ));

        // Update C3 to use this formula
        grid.get_node(3, 3).function = func1.clone();
        update_edges(&mut grid, c3.clone(), func1.clone(), true);

        // Verify dependencies
        assert!(grid.get_node(1, 1).dependents.contains(&c3));
        assert!(grid.get_node(2, 2).dependents.contains(&c3));

        // Check for cycles (should be none)
        assert!(!has_cycle(&mut grid, c3.clone()));

        // Get evaluation sequence
        let sequence = get_sequence(&mut grid, c3.clone());
        assert_eq!(sequence.len(), 1);
        assert_eq!(sequence[0], c3);

        // Now change formula: C3 = D4 * 5
        let func2 = Some(Value::Oper(
            Some(Box::new(d4.clone())),
            Some(Box::new(Value::Const(5))),
            Operation::Mul,
        ));

        // Update C3 with new formula
        grid.get_node(3, 3).function = func2.clone();
        update_edges(&mut grid, c3.clone(), func2.clone(), true);

        // Verify old dependencies are removed
        assert!(grid.get_node(1, 1).dependents.contains(&c3));
        assert!(grid.get_node(2, 2).dependents.contains(&c3));

        // Verify new dependency is added
        assert!(grid.get_node(4, 4).dependents.contains(&c3));

        // Check for cycles (should be none)
        assert!(!has_cycle(&mut grid, c3.clone()));

        // Try to create a cycle: D4 = C3 (should fail)
        let cycle_func = Some(Value::Oper(
            Some(Box::new(c3.clone())),
            Some(Box::new(Value::Const(2))),
            Operation::Mul,
        ));

        // Update D4 with formula that would create a cycle
        grid.get_node(4, 4).function = cycle_func.clone();
        add_edges(&mut grid, d4.clone(), cycle_func.clone(), true);

        // Verify cycle is detected
        assert!(has_cycle(&mut grid, d4.clone()));

        // Break the cycle
        break_edges(&mut grid, d4.clone(), cycle_func.clone(), true);

        // Verify cycle is gone
        assert!(!has_cycle(&mut grid, d4.clone()));
    }
}
