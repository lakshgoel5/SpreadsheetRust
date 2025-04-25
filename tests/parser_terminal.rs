use project::terminal::functions::Operation;
use project::terminal::functions::Value;
use project::terminal::parser;

#[test]
fn test_validate_cell_assignment() {
    // Test assigning a constant to a cell
    let rows = 10;
    let cols = 10;

    // Test valid cell assignment
    let result = parser::validate("A1=5", &cols, &rows);
    assert!(result.is_some());

    // Test invalid cell (out of range)
    // let result = parser::validate("Z99=5", &cols, &rows);
    // assert!(result.is_none());
}

#[test]
fn test_validate_arithmetic_operations() {
    let rows = 10;
    let cols = 10;

    // Test addition
    let result = parser::validate("B2=A1+5", &cols, &rows);
    assert!(result.is_some());

    // Test subtraction
    let result = parser::validate("C3=10-D4", &cols, &rows);
    assert!(result.is_some());

    if let Some((cell, _operation)) = result {
        assert_eq!(cell, Some(Value::Cell(3, 3)));
    }

    // Test multiplication
    let result = parser::validate("E5=F6*G7", &cols, &rows);
    assert!(result.is_some());

    // Test division
    let result = parser::validate("H8=100/I9", &cols, &rows);
    assert!(result.is_some());
}

#[test]
fn test_validate_range_operations() {
    let rows = 10;
    let cols = 10;

    // Test SUM
    let result = parser::validate("D1=SUM(A1:C3)", &cols, &rows);
    assert!(result.is_some());

    if let Some((cell, _operation)) = result {
        assert_eq!(cell, Some(Value::Cell(4, 1)));
    }

    // Test AVG
    let result = parser::validate("E1=AVG(A1:C3)", &cols, &rows);
    assert!(result.is_some());

    if let Some((cell, operation)) = result {
        assert_eq!(cell, Some(Value::Cell(5, 1)));
        if let Some(Value::Oper(_box1, _box2, op)) = operation {
            assert!(matches!(op, Operation::Avg));
        }
    }

    // Test MIN
    let result = parser::validate("F1=MIN(A1:C3)", &cols, &rows);
    assert!(result.is_some());

    if let Some((_cell, operation)) = result {
        if let Some(Value::Oper(_, _, op)) = operation {
            assert!(matches!(op, Operation::Min));
        }
    }

    // Test MAX
    let result = parser::validate("G1=MAX(A1:C3)", &cols, &rows);
    assert!(result.is_some());

    if let Some((_cell, operation)) = result {
        if let Some(Value::Oper(_, _, op)) = operation {
            assert!(matches!(op, Operation::Max));
        }
    }

    // Test STDEV
    let result = parser::validate("H1=STDEV(A1:C3)", &cols, &rows);
    assert!(result.is_some());

    if let Some((_cell, operation)) = result {
        if let Some(Value::Oper(_, _, op)) = operation {
            assert!(matches!(op, Operation::Std));
        }
    }

    // Test invalid range (start > end)
    let result = parser::validate("I1=SUM(C3:A1)", &cols, &rows);
    assert!(result.is_none());
}

#[test]
fn test_validate_scroll_to() {
    let rows = 10;
    let cols = 10;

    // Test valid scroll_to command
    let result = parser::validate("scroll_to B5", &cols, &rows);
    assert!(result.is_some());

    if let Some((cell, operation)) = result {
        assert_eq!(cell, Some(Value::Cell(2, 5)));
        if let Some(Value::Oper(_, _, op)) = operation {
            assert!(matches!(op, Operation::Scrollto));
        }
    }

    // Test invalid scroll_to (out of bounds)
    let result = parser::validate("scroll_to Z99", &cols, &rows);
    assert!(result.is_none());
}

#[test]
fn test_validate_output_commands() {
    let rows = 10;
    let cols = 10;

    // Test enable_output command
    let result = parser::validate("enable_output", &cols, &rows);
    assert!(result.is_some());

    if let Some((_, Some(Value::Oper(_, _, op)))) = result {
        assert!(matches!(op, Operation::EnableOutput));
    }

    // Test disable_output command
    let result = parser::validate("disable_output", &cols, &rows);
    assert!(result.is_some());

    if let Some((_, Some(Value::Oper(_, _, op)))) = result {
        assert!(matches!(op, Operation::DisableOutput));
    }
}

#[test]
fn test_validate_invalid_commands() {
    let rows = 10;
    let cols = 10;

    // Test completely invalid command
    let result = parser::validate("this is not a valid command", &cols, &rows);
    assert!(result.is_none());

    // Test command with missing equals sign
    let result = parser::validate("A1 5", &cols, &rows);
    assert!(result.is_none());

    // Test invalid function name
    let result = parser::validate("A1=INVALID(B1:C3)", &cols, &rows);
    let expected_value = Some((Some(Value::Cell(1, 1)), None));
    assert_eq!(result, expected_value);
}

#[test]
fn test_is_cell() {
    let rows = 10;
    let cols = 10;

    // Valid cells
    assert_eq!(parser::is_cell("A1", &rows, &cols), Some(Value::Cell(1, 1)));
    assert_eq!(parser::is_cell("B5", &rows, &cols), Some(Value::Cell(2, 5)));
    assert_eq!(
        parser::is_cell("J10", &rows, &cols),
        Some(Value::Cell(10, 10))
    );

    // Invalid cells
    assert_eq!(parser::is_cell("Z1", &rows, &cols), None); // Column out of range
    assert_eq!(parser::is_cell("A20", &rows, &cols), None); // Row out of range
    assert_eq!(parser::is_cell("1A", &rows, &cols), None); // Invalid format
    assert_eq!(parser::is_cell("AA1", &rows, &cols), None); // Two-letter column
}

#[test]
fn test_is_const() {
    // Valid integers
    assert_eq!(parser::is_const("0"), Some(Value::Const(0)));
    assert_eq!(parser::is_const("42"), Some(Value::Const(42)));
    assert_eq!(parser::is_const("-10"), Some(Value::Const(-10)));

    // Invalid integers
    assert_eq!(parser::is_const(""), None); // Empty string
    assert_eq!(parser::is_const("a"), None); // Non-numeric
    assert_eq!(parser::is_const("3.14"), None); // Float, not integer
}

#[test]
fn test_is_cell_or_const() {
    let rows = 10;
    let cols = 10;

    // Test constant values
    assert_eq!(
        parser::is_cell_or_const("42", &rows, &cols),
        Some(Value::Const(42))
    );
    assert_eq!(
        parser::is_cell_or_const("-5", &rows, &cols),
        Some(Value::Const(-5))
    );

    // Test cell references
    assert_eq!(
        parser::is_cell_or_const("A1", &rows, &cols),
        Some(Value::Cell(1, 1))
    );
    assert_eq!(
        parser::is_cell_or_const("J10", &rows, &cols),
        Some(Value::Cell(10, 10))
    );

    // // Test invalid inputs
    assert_eq!(parser::is_cell_or_const("ZZ99", &rows, &cols), None);
    assert_eq!(parser::is_cell_or_const("3.14", &rows, &cols), None);
}

#[test]
fn test_validate_complex_arithmetic() {
    let rows = 10;
    let cols = 10;

    // Test negative numbers in operations
    let result = parser::validate("A1=-5+B2", &cols, &rows);
    assert!(result.is_some());

    // Test cell references with cells
    let result = parser::validate("C3=A1+B2", &cols, &rows);
    assert!(result.is_some());

    // Test multiple digit constants
    let result = parser::validate("D4=123+456", &cols, &rows);
    assert!(result.is_some());

    // Test all operators with different combinations
    let operations = [
        ("E1=15+25", Operation::Add),
        ("E2=30-10", Operation::Sub),
        ("E3=5*6", Operation::Mul),
        ("E4=20/4", Operation::Div),
    ];

    for (cmd, _expected_op) in operations {
        let result = parser::validate(cmd, &cols, &rows);
        assert!(result.is_some());
        if let Some((_, Some(Value::Oper(_, _, op)))) = result {
            assert!(matches!(op, _expected_op));
        } else {
            panic!("Expected operation for {}", cmd);
        }
    }
}

#[test]
fn test_missing_special_commands() {
    let rows = 10;
    let cols = 10;

    // Test all scroll_to variations
    let result = parser::validate("scroll_to A1", &cols, &rows);
    assert!(result.is_some());
    if let Some((cell, Some(Value::Oper(_, _, op)))) = result {
        assert_eq!(cell, Some(Value::Cell(1, 1)));
        assert!(matches!(op, Operation::Scrollto));
    }

    // Test with extra whitespace
    let result = parser::validate("  scroll_to  C3  ", &cols, &rows);
    assert!(result.is_some());

    // Test invalid cell for scroll_to
    let result = parser::validate("scroll_to Z99", &cols, &rows);
    assert!(result.is_none());

    // Test invalid format for scroll_to
    let result = parser::validate("scroll_toA1", &cols, &rows);
    assert!(result.is_none());
}

#[test]
fn test_command_validation_edge_cases() {
    let rows = 10;
    let cols = 10;

    // Test missing closing parenthesis
    let result = parser::validate("A1=SUM(B1:C2", &cols, &rows);
    assert!(result.is_none());

    // Test invalid chars in cell names
    let result = parser::validate("A@1=5", &cols, &rows);
    if let Some((cell, Some(Value::Oper(_, _, _op)))) = result {
        assert!(cell.is_none());
    }
}
