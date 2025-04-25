use project::terminal::functions::Operation;
use project::terminal::graph::Node;
use project::terminal::types::Coordinates;

#[test]
fn test_node_get_value() {
    let node = Node {
        dependents: Vec::new(),
        node_value: 42,
        value1: Coordinates { row: 0, col: 0 },
        value2: Coordinates { row: 0, col: 0 },
        position: Coordinates { row: 1, col: 1 },
        op: Operation::Add,
        valid: true,
        visited: false,
    };

    assert_eq!(node.get_value(), 42);
}

#[test]
fn test_node_set_value() {
    let mut node = Node {
        dependents: Vec::new(),
        node_value: 42,
        value1: Coordinates { row: 0, col: 0 },
        value2: Coordinates { row: 0, col: 0 },
        position: Coordinates { row: 1, col: 1 },
        op: Operation::Add,
        valid: true,
        visited: false,
    };

    node.set_value(99);
    assert_eq!(node.node_value, 99);
    assert_eq!(node.get_value(), 99);
}

#[test]
fn test_node_set_position() {
    let mut node = Node {
        dependents: Vec::new(),
        node_value: 42,
        value1: Coordinates { row: 0, col: 0 },
        value2: Coordinates { row: 0, col: 0 },
        position: Coordinates { row: 1, col: 1 },
        op: Operation::Add,
        valid: true,
        visited: false,
    };

    let new_position = Coordinates { row: 3, col: 4 };
    node.set_position(new_position);

    assert_eq!(node.position.row, 3);
    assert_eq!(node.position.col, 4);
}

#[test]
fn test_node_set_value1() {
    let mut node = Node {
        dependents: Vec::new(),
        node_value: 42,
        value1: Coordinates { row: 0, col: 0 },
        value2: Coordinates { row: 0, col: 0 },
        position: Coordinates { row: 1, col: 1 },
        op: Operation::Add,
        valid: true,
        visited: false,
    };

    let new_value1 = Coordinates { row: 5, col: 6 };
    node.set_value1(new_value1);

    assert_eq!(node.value1.row, 5);
    assert_eq!(node.value1.col, 6);
}

#[test]
fn test_node_set_value2() {
    let mut node = Node {
        dependents: Vec::new(),
        node_value: 42,
        value1: Coordinates { row: 0, col: 0 },
        value2: Coordinates { row: 0, col: 0 },
        position: Coordinates { row: 1, col: 1 },
        op: Operation::Add,
        valid: true,
        visited: false,
    };

    let new_value2 = Coordinates { row: 7, col: 8 };
    node.set_value2(new_value2);

    assert_eq!(node.value2.row, 7);
    assert_eq!(node.value2.col, 8);
}

#[test]
fn test_node_get_valid() {
    let valid_node = Node {
        dependents: Vec::new(),
        node_value: 42,
        value1: Coordinates { row: 0, col: 0 },
        value2: Coordinates { row: 0, col: 0 },
        position: Coordinates { row: 1, col: 1 },
        op: Operation::Add,
        valid: true,
        visited: false,
    };

    let invalid_node = Node {
        dependents: Vec::new(),
        node_value: 42,
        value1: Coordinates { row: 0, col: 0 },
        value2: Coordinates { row: 0, col: 0 },
        position: Coordinates { row: 1, col: 1 },
        op: Operation::Add,
        valid: false,
        visited: false,
    };

    assert!(valid_node.get_valid());
    assert!(!invalid_node.get_valid());
}

#[test]
fn test_node_set_valid() {
    let mut node = Node {
        dependents: Vec::new(),
        node_value: 42,
        value1: Coordinates { row: 0, col: 0 },
        value2: Coordinates { row: 0, col: 0 },
        position: Coordinates { row: 1, col: 1 },
        op: Operation::Add,
        valid: true,
        visited: false,
    };

    assert!(node.get_valid());

    node.set_valid(false);
    assert!(!node.get_valid());
    assert!(!node.valid);

    node.set_valid(true);
    assert!(node.get_valid());
    assert!(node.valid);
}

#[test]
fn test_node_add_dep() {
    let mut node = Node {
        dependents: Vec::new(),
        node_value: 42,
        value1: Coordinates { row: 0, col: 0 },
        value2: Coordinates { row: 0, col: 0 },
        position: Coordinates { row: 1, col: 1 },
        op: Operation::Add,
        valid: true,
        visited: false,
    };

    // Add first dependent
    let dep1 = Coordinates { row: 2, col: 3 };
    node.add_dep(dep1);

    assert_eq!(node.dependents.len(), 1);
    assert_eq!(node.dependents[0].row, 2);
    assert_eq!(node.dependents[0].col, 3);

    // Add second dependent
    let dep2 = Coordinates { row: 4, col: 5 };
    node.add_dep(dep2);

    assert_eq!(node.dependents.len(), 2);
    assert_eq!(node.dependents[1].row, 4);
    assert_eq!(node.dependents[1].col, 5);

    // Try to add duplicate dependent
    node.add_dep(dep1);

    // Should still have only 2 dependents (no duplicates)
    assert_eq!(node.dependents.len(), 2);
}

#[test]
fn test_node_remove_dep() {
    let mut node = Node {
        dependents: Vec::new(),
        node_value: 42,
        value1: Coordinates { row: 0, col: 0 },
        value2: Coordinates { row: 0, col: 0 },
        position: Coordinates { row: 1, col: 1 },
        op: Operation::Add,
        valid: true,
        visited: false,
    };

    // Add dependents
    let dep1 = Coordinates { row: 2, col: 3 };
    let dep2 = Coordinates { row: 4, col: 5 };
    let dep3 = Coordinates { row: 6, col: 7 };

    node.add_dep(dep1);
    node.add_dep(dep2);
    node.add_dep(dep3);

    assert_eq!(node.dependents.len(), 3);

    // Remove a dependent
    node.remove_dep(dep2);

    assert_eq!(node.dependents.len(), 2);
    assert_eq!(node.dependents[0].row, 2);
    assert_eq!(node.dependents[0].col, 3);
    assert_eq!(node.dependents[1].row, 6);
    assert_eq!(node.dependents[1].col, 7);

    // Try to remove a dependent that doesn't exist
    let non_existent_dep = Coordinates { row: 8, col: 9 };
    node.remove_dep(non_existent_dep);

    // Should still have 2 dependents
    assert_eq!(node.dependents.len(), 2);
}

#[test]
fn test_node_get_dependents() {
    let mut node = Node {
        dependents: Vec::new(),
        node_value: 42,
        value1: Coordinates { row: 0, col: 0 },
        value2: Coordinates { row: 0, col: 0 },
        position: Coordinates { row: 1, col: 1 },
        op: Operation::Add,
        valid: true,
        visited: false,
    };

    // Add dependents
    let dep1 = Coordinates { row: 2, col: 3 };
    let dep2 = Coordinates { row: 4, col: 5 };

    node.add_dep(dep1);
    node.add_dep(dep2);

    let deps = node.get_dependents();

    assert_eq!(deps.len(), 2);
    assert_eq!(deps[0].row, 2);
    assert_eq!(deps[0].col, 3);
    assert_eq!(deps[1].row, 4);
    assert_eq!(deps[1].col, 5);
}

#[test]
fn test_node_set_dependents() {
    let mut node = Node {
        dependents: Vec::new(),
        node_value: 42,
        value1: Coordinates { row: 0, col: 0 },
        value2: Coordinates { row: 0, col: 0 },
        position: Coordinates { row: 1, col: 1 },
        op: Operation::Add,
        valid: true,
        visited: false,
    };

    // Create a list of dependents
    let mut deps = Vec::new();
    deps.push(Coordinates { row: 2, col: 3 });
    deps.push(Coordinates { row: 4, col: 5 });
    deps.push(Coordinates { row: 6, col: 7 });

    // Set the dependents
    node.set_dependents(deps);

    assert_eq!(node.dependents.len(), 3);
    assert_eq!(node.dependents[0].row, 2);
    assert_eq!(node.dependents[0].col, 3);
    assert_eq!(node.dependents[1].row, 4);
    assert_eq!(node.dependents[1].col, 5);
    assert_eq!(node.dependents[2].row, 6);
    assert_eq!(node.dependents[2].col, 7);

    // Override with a new list
    let mut new_deps = Vec::new();
    new_deps.push(Coordinates { row: 8, col: 9 });

    node.set_dependents(new_deps);

    assert_eq!(node.dependents.len(), 1);
    assert_eq!(node.dependents[0].row, 8);
    assert_eq!(node.dependents[0].col, 9);
}

// Helper function to create a default Node for testing
fn create_test_node() -> Node {
    Node {
        dependents: Vec::new(),
        node_value: 0,
        value1: Coordinates { row: 0, col: 0 },
        value2: Coordinates { row: 0, col: 0 },
        position: Coordinates { row: 0, col: 0 },
        op: Operation::Add,
        valid: true,
        visited: false,
    }
}

#[test]
fn test_node_combined_operations() {
    let mut node = create_test_node();

    // Test multiple operations in sequence
    node.set_value(42);
    assert_eq!(node.get_value(), 42);

    node.set_position(Coordinates { row: 1, col: 2 });
    assert_eq!(node.position.row, 1);
    assert_eq!(node.position.col, 2);

    node.add_dep(Coordinates { row: 3, col: 4 });
    node.add_dep(Coordinates { row: 5, col: 6 });
    assert_eq!(node.dependents.len(), 2);

    node.remove_dep(Coordinates { row: 3, col: 4 });
    assert_eq!(node.dependents.len(), 1);
    assert_eq!(node.dependents[0].row, 5);
    assert_eq!(node.dependents[0].col, 6);

    node.set_valid(false);
    assert!(!node.get_valid());

    // Test that setting dependents replaces existing ones
    let mut new_deps = Vec::new();
    new_deps.push(Coordinates { row: 7, col: 8 });
    new_deps.push(Coordinates { row: 9, col: 10 });

    node.set_dependents(new_deps);
    assert_eq!(node.get_dependents().len(), 2);
    assert_eq!(node.get_dependents()[0].row, 7);
    assert_eq!(node.get_dependents()[0].col, 8);
    assert_eq!(node.get_dependents()[1].row, 9);
    assert_eq!(node.get_dependents()[1].col, 10);
}
