use spreadsheet_rust::extension::common::{Operation, Value};
use spreadsheet_rust::extension::parser::parser;

#[test]
fn test_validate_cell_assignment() {
    // Test assigning a constant to a cell
    let rows = 10;
    let cols = 10;
    
    // Test valid cell assignment
    let result = parser::validate("A1=5", &cols, &rows);
    assert!(result.is_some());
    
    if let Some((cell, operation)) = result {
        assert_eq!(cell, Some(Value::Cell(1, 1)));
        if let Some(Value::Oper(box1, box2, op)) = operation {
            assert_eq!(*box1.unwrap(), Value::Const(5));
            assert_eq!(*box2.unwrap(), Value::Const(0));
            assert!(matches!(op, Operation::Cons));
        } else {
            panic!("Expected Value::Oper for operation");
        }
    }
    
    // Test invalid cell (out of range)
    let result = parser::validate("Z99=5", &cols, &rows);
    assert!(result.is_none());
}

#[test]
fn test_validate_arithmetic_operations() {
    let rows = 10;
    let cols = 10;
    
    // Test addition
    let result = parser::validate("B2=A1+5", &cols, &rows);
    assert!(result.is_some());
    
    if let Some((cell, operation)) = result {
        assert_eq!(cell, Some(Value::Cell(2, 2)));
        if let Some(Value::Oper(box1, box2, op)) = operation {
            assert_eq!(*box1.unwrap(), Value::Cell(1, 1));
            assert_eq!(*box2.unwrap(), Value::Const(5));
            assert!(matches!(op, Operation::Add));
        }
    }
    
    // Test subtraction
    let result = parser::validate("C3=10-D4", &cols, &rows);
    assert!(result.is_some());
    
    if let Some((cell, operation)) = result {
        assert_eq!(cell, Some(Value::Cell(3, 3)));
        if let Some(Value::Oper(box1, box2, op)) = operation {
            assert_eq!(*box1.unwrap(), Value::Const(10));
            assert_eq!(*box2.unwrap(), Value::Cell(4, 4));
            assert!(matches!(op, Operation::Sub));
        }
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
    
    if let Some((cell, operation)) = result {
        assert_eq!(cell, Some(Value::Cell(1, 4)));
        if let Some(Value::Oper(box1, box2, op)) = operation {
            assert_eq!(*box1.unwrap(), Value::Cell(1, 1));
            assert_eq!(*box2.unwrap(), Value::Cell(3, 3));
            assert!(matches!(op, Operation::Sum));
        }
    }
    
    // Test AVG
    let result = parser::validate("E1=AVG(A1:C3)", &cols, &rows);
    assert!(result.is_some());
    
    if let Some((cell, operation)) = result {
        assert_eq!(cell, Some(Value::Cell(1, 5)));
        if let Some(Value::Oper(box1, box2, op)) = operation {
            assert!(matches!(op, Operation::Avg));
        }
    }
    
    // Test MIN
    let result = parser::validate("F1=MIN(A1:C3)", &cols, &rows);
    assert!(result.is_some());
    
    if let Some((cell, operation)) = result {
        if let Some(Value::Oper(_, _, op)) = operation {
            assert!(matches!(op, Operation::Min));
        }
    }
    
    // Test MAX
    let result = parser::validate("G1=MAX(A1:C3)", &cols, &rows);
    assert!(result.is_some());
    
    if let Some((cell, operation)) = result {
        if let Some(Value::Oper(_, _, op)) = operation {
            assert!(matches!(op, Operation::Max));
        }
    }
    
    // Test STDEV
    let result = parser::validate("H1=STDEV(A1:C3)", &cols, &rows);
    assert!(result.is_some());
    
    if let Some((cell, operation)) = result {
        if let Some(Value::Oper(_, _, op)) = operation {
            assert!(matches!(op, Operation::Std));
        }
    }
    
    // Test invalid range (start > end)
    let result = parser::validate("I1=SUM(C3:A1)", &cols, &rows);
    assert!(result.is_none());
}

#[test]
fn test_validate_special_commands() {
    let rows = 10;
    let cols = 10;
    
    // Test undo command
    let result = parser::validate("undo", &cols, &rows);
    assert!(result.is_some());
    
    if let Some((cell, operation)) = result {
        assert!(cell.is_none());
        if let Some(Value::Oper(box1, box2, op)) = operation {
            assert!(box1.is_none());
            assert!(box2.is_none());
            assert!(matches!(op, Operation::Undo));
        }
    }
    
    // Test redo command
    let result = parser::validate("redo", &cols, &rows);
    assert!(result.is_some());
    
    if let Some((cell, operation)) = result {
        assert!(cell.is_none());
        if let Some(Value::Oper(_, _, op)) = operation {
            assert!(matches!(op, Operation::Redo));
        }
    }
    
    // Test directional commands
    let result = parser::validate("w", &cols, &rows);
    assert!(result.is_some());
    if let Some((_, Some(Value::Oper(_, _, op)))) = result {
        assert!(matches!(op, Operation::Up));
    }
    
    let result = parser::validate("s", &cols, &rows);
    assert!(result.is_some());
    if let Some((_, Some(Value::Oper(_, _, op)))) = result {
        assert!(matches!(op, Operation::Down));
    }
    
    let result = parser::validate("a", &cols, &rows);
    assert!(result.is_some());
    if let Some((_, Some(Value::Oper(_, _, op)))) = result {
        assert!(matches!(op, Operation::Left));
    }
    
    let result = parser::validate("d", &cols, &rows);
    assert!(result.is_some());
    if let Some((_, Some(Value::Oper(_, _, op)))) = result {
        assert!(matches!(op, Operation::Right));
    }
    
    // Test quit command
    let result = parser::validate("q", &cols, &rows);
    assert!(result.is_some());
    if let Some((_, Some(Value::Oper(_, _, op)))) = result {
        assert!(matches!(op, Operation::Quit));
    }
}

#[test]
fn test_validate_save_web_commands() {
    let rows = 10;
    let cols = 10;
    
    // Test save command
    let result = parser::validate("save myfile.json", &cols, &rows);
    assert!(result.is_some());
    
    if let Some((cell, operation)) = result {
        assert!(cell.is_none());
        if let Some(Value::Oper(box1, box2, Operation::Save(filename))) = operation {
            assert!(box1.is_none());
            assert!(box2.is_none());
            assert_eq!(filename, "myfile.json");
        }
    }
    
    // Test web command
    let result = parser::validate("web index.html", &cols, &rows);
    assert!(result.is_some());
    
    if let Some((cell, operation)) = result {
        assert!(cell.is_none());
        if let Some(Value::Oper(box1, box2, Operation::Web(filename))) = operation {
            assert!(box1.is_none());
            assert!(box2.is_none());
            assert_eq!(filename, "index.html");
        }
    }
    
    // Test web_start command
    let result = parser::validate("web_start", &cols, &rows);
    assert!(result.is_some());
    
    if let Some((_, Some(Value::Oper(_, _, op)))) = result {
        assert!(matches!(op, Operation::WebStart));
    }
}

#[test]
fn test_validate_scroll_to() {
    let rows = 10;
    let cols = 10;
    
    // Test valid scroll_to command
    let result = parser::validate("scroll_to B5", &cols, &rows);
    assert!(result.is_some());
    
    if let Some((cell, operation)) = result {
        assert_eq!(cell, Some(Value::Cell(5, 2)));
        if let Some(Value::Oper(_, _, op)) = operation {
            assert!(matches!(op, Operation::ScrollTo));
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

