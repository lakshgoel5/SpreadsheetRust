use project::extension::backend::backend::*;
use project::extension::backend::functions::*;
use project::extension::backend::graph::*;
#[allow(unused_imports)]
use project::extension::common::{Operation, Value};

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_grid(rows: usize, cols: usize) -> Grid {
        let mut grid = Grid::new(rows, cols);
        // Initialize grid with default values
        for i in 0..rows {
            for j in 0..cols {
                let node = Node::new(i as isize * cols as isize + j as isize);
                grid.set_node(i, j, node);
            }
        }
        grid
    }

    fn setup_grid_with_range_operation(rows: usize, cols: usize, oper: Operation) -> Grid {
        let mut grid = setup_test_grid(rows, cols);

        // Set up a cell with a range operation
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Cell(0, 0))),
            Some(Box::new(Value::Cell(2, 2))),
            oper,
        ));
        grid.set_node(3, 3, function_node);

        grid
    }

    fn setup_grid_with_binary_operation(rows: usize, cols: usize, oper: Operation) -> Grid {
        let mut grid = setup_test_grid(rows, cols);

        // Set up a cell with a binary operation
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Cell(1, 1))),
            Some(Box::new(Value::Cell(2, 2))),
            oper,
        ));
        grid.set_node(3, 3, function_node);

        grid
    }

    fn setup_grid_with_const_operation(rows: usize, cols: usize, oper: Operation) -> Grid {
        let mut grid = setup_test_grid(rows, cols);

        // Set up a cell with a binary operation using constants
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Const(10))),
            Some(Box::new(Value::Const(5))),
            oper,
        ));
        grid.set_node(3, 3, function_node);

        grid
    }

    fn setup_grid_with_mixed_operation(rows: usize, cols: usize, oper: Operation) -> Grid {
        let mut grid = setup_test_grid(rows, cols);

        // Set up a cell with a binary operation using both cell and constant
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Cell(1, 1))),
            Some(Box::new(Value::Const(5))),
            oper,
        ));
        grid.set_node(3, 3, function_node);

        grid
    }

    fn setup_invalid_grid_in_range(rows: usize, cols: usize, oper: Operation) -> Grid {
        let mut grid = setup_test_grid(rows, cols);

        // Set up a cell with a range operation
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Cell(0, 0))),
            Some(Box::new(Value::Cell(2, 2))),
            oper,
        ));
        grid.set_node(3, 3, function_node);

        // Make one cell in the range invalid
        let invalid_node = grid.get_node(1, 1);
        invalid_node.valid = false;

        grid
    }

    #[test]
    fn test_max_function_normal() {
        let mut grid = setup_grid_with_range_operation(5, 5, Operation::Max);

        // In a 3x3 grid (0,0 to 2,2), the max value would be at (2,2) = 2*5+2 = 12
        let result = max_function(&mut grid, 3, 3);
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_max_function_empty_range() {
        let mut grid = setup_test_grid(5, 5);

        // Set up a cell with a range operation for an empty range
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Cell(2, 2))),
            Some(Box::new(Value::Cell(1, 1))),
            Operation::Max,
        ));
        grid.set_node(3, 3, function_node);

        // For an empty range (since end < start), should return None
        let result = max_function(&mut grid, 3, 3);
        assert_eq!(result, Some(isize::MIN));
    }

    #[test]
    fn test_max_function_invalid_cell() {
        let mut grid = setup_invalid_grid_in_range(5, 5, Operation::Max);

        // If there's an invalid cell in the range, should return None
        let result = max_function(&mut grid, 3, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_max_function_invalid_function() {
        let mut grid = setup_test_grid(5, 5);

        // Set up a cell with an invalid function structure
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Cell(1, 1));
        grid.set_node(3, 3, function_node);

        // With an invalid function structure, should return None
        let result = max_function(&mut grid, 3, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_min_function_normal() {
        let mut grid = setup_grid_with_range_operation(5, 5, Operation::Min);

        // In a 3x3 grid (0,0 to 2,2), the min value would be at (0,0) = 0
        let result = min_function(&mut grid, 3, 3);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_min_function_empty_range() {
        let mut grid = setup_test_grid(5, 5);

        // Set up a cell with a range operation for an empty range
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Cell(2, 2))),
            Some(Box::new(Value::Cell(1, 1))),
            Operation::Min,
        ));
        grid.set_node(3, 3, function_node);

        // For an empty range (since end < start), should return None
        let result = min_function(&mut grid, 3, 3);
        assert_eq!(result, Some(isize::MAX));
    }

    #[test]
    fn test_min_function_invalid_cell() {
        let mut grid = setup_invalid_grid_in_range(5, 5, Operation::Min);

        // If there's an invalid cell in the range, should return None
        let result = min_function(&mut grid, 3, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_sum_function_normal() {
        let mut grid = setup_grid_with_range_operation(5, 5, Operation::Sum);

        // In a 3x3 grid (0,0 to 2,2), the sum would be 0+1+2+5+6+7+10+11+12 = 54
        let result = sum_function(&mut grid, 3, 3);
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_sum_function_empty_range() {
        let mut grid = setup_test_grid(5, 5);

        // Set up a cell with a range operation for an empty range
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Cell(2, 2))),
            Some(Box::new(Value::Cell(1, 1))),
            Operation::Sum,
        ));
        grid.set_node(3, 3, function_node);

        // For an empty range (since end < start), should return Some(0)
        let result = sum_function(&mut grid, 3, 3);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_sum_function_invalid_cell() {
        let mut grid = setup_invalid_grid_in_range(5, 5, Operation::Sum);

        // If there's an invalid cell in the range, should return None
        let result = sum_function(&mut grid, 3, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_avg_function_normal() {
        let mut grid = setup_grid_with_range_operation(5, 5, Operation::Avg);

        // In a 3x3 grid (0,0 to 2,2), the average would be (0+1+2+5+6+7+10+11+12)/9 = 54/9 = 6
        let result = avg_function(&mut grid, 3, 3);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_avg_function_empty_range() {
        let mut grid = setup_test_grid(5, 5);

        // Set up a cell with a range operation for an empty range
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Cell(2, 2))),
            Some(Box::new(Value::Cell(1, 1))),
            Operation::Avg,
        ));
        grid.set_node(3, 3, function_node);

        // For an empty range (since end < start), should return None
        let result = avg_function(&mut grid, 3, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_avg_function_invalid_cell() {
        let mut grid = setup_invalid_grid_in_range(5, 5, Operation::Avg);

        // If there's an invalid cell in the range, should return None
        let result = avg_function(&mut grid, 3, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_std_dev_function_normal() {
        let mut grid = setup_grid_with_range_operation(5, 5, Operation::Std);

        // In a 3x3 grid (0,0 to 2,2), values are 0,1,2,5,6,7,10,11,12
        // Mean is 6, variance is (6-0)²+(6-1)²+(6-2)²+(6-5)²+(6-6)²+(6-7)²+(6-10)²+(6-11)²+(6-12)² = 36+25+16+1+0+1+16+25+36 = 156
        // StdDev is sqrt(156/9) = sqrt(17.333) ≈ 4.16 rounded to 4
        let result = std_dev_function(&mut grid, 3, 3);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_std_dev_function_empty_range() {
        let mut grid = setup_test_grid(5, 5);

        // Set up a cell with a range operation for an empty range
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Cell(2, 2))),
            Some(Box::new(Value::Cell(1, 1))),
            Operation::Std,
        ));
        grid.set_node(3, 3, function_node);

        // For an empty range (since end < start), should return Some(0)
        let result = std_dev_function(&mut grid, 3, 3);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_std_dev_function_invalid_cell() {
        let mut grid = setup_invalid_grid_in_range(5, 5, Operation::Std);

        // If there's an invalid cell in the range, should return None
        let result = std_dev_function(&mut grid, 3, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_add_normal() {
        let mut grid = setup_grid_with_binary_operation(5, 5, Operation::Add);

        // Adding cell(1,1) = 6 and cell(2,2) = 12
        let result = add(&mut grid, 3, 3);
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_add_with_const() {
        let mut grid = setup_grid_with_const_operation(5, 5, Operation::Add);

        // Adding constants 10 + 5
        let result = add(&mut grid, 3, 3);
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_add_with_mixed() {
        let mut grid = setup_grid_with_mixed_operation(5, 5, Operation::Add);

        // Adding cell(1,1) = 6 and const 5
        let result = add(&mut grid, 3, 3);
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_add_invalid_cell() {
        let mut grid = setup_grid_with_binary_operation(5, 5, Operation::Add);

        // Make one of the cells invalid
        let invalid_node = grid.get_node(1, 1);
        invalid_node.valid = false;

        // Should return None if one of the cells is invalid
        let result = add(&mut grid, 3, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_add_invalid_function() {
        let mut grid = setup_test_grid(5, 5);

        // Set up a cell with an invalid function structure
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Cell(1, 1));
        grid.set_node(3, 3, function_node);

        // With an invalid function structure, should return None
        let result = add(&mut grid, 3, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_sub_normal() {
        let mut grid = setup_grid_with_binary_operation(5, 5, Operation::Sub);

        // Subtracting cell(1,1) = 6 from cell(2,2) = 12
        let result = sub(&mut grid, 3, 3);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_sub_with_const() {
        let mut grid = setup_grid_with_const_operation(5, 5, Operation::Sub);

        // Subtracting constants 10 - 5
        let result = sub(&mut grid, 3, 3);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_sub_with_mixed() {
        let mut grid = setup_grid_with_mixed_operation(5, 5, Operation::Sub);

        // Subtracting cell(1,1) = 6 and const 5
        let result = sub(&mut grid, 3, 3);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_sub_invalid_cell() {
        let mut grid = setup_grid_with_binary_operation(5, 5, Operation::Sub);

        // Make one of the cells invalid
        let invalid_node = grid.get_node(2, 2);
        invalid_node.valid = false;

        // Should return None if one of the cells is invalid
        let result = sub(&mut grid, 3, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_mul_normal() {
        let mut grid = setup_grid_with_binary_operation(5, 5, Operation::Mul);

        // Multiplying cell(1,1) = 6 and cell(2,2) = 12
        let result = mul(&mut grid, 3, 3);
        assert_eq!(result, Some(72));
    }

    #[test]
    fn test_mul_with_const() {
        let mut grid = setup_grid_with_const_operation(5, 5, Operation::Mul);

        // Multiplying constants 10 * 5
        let result = mul(&mut grid, 3, 3);
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_mul_with_mixed() {
        let mut grid = setup_grid_with_mixed_operation(5, 5, Operation::Mul);

        // Multiplying cell(1,1) = 6 and const 5
        let result = mul(&mut grid, 3, 3);
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_mul_invalid_cell() {
        let mut grid = setup_grid_with_binary_operation(5, 5, Operation::Mul);

        // Make one of the cells invalid
        let invalid_node = grid.get_node(1, 1);
        invalid_node.valid = false;

        // Should return None if one of the cells is invalid
        let result = mul(&mut grid, 3, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_div_normal() {
        let mut grid = setup_grid_with_binary_operation(5, 5, Operation::Div);

        // Dividing cell(1,1) = 6 by cell(2,2) = 12
        let result = div(&mut grid, 3, 3);
        assert_eq!(result, Some(0)); // Integer division
    }

    #[test]
    fn test_div_with_const() {
        let mut grid = setup_grid_with_const_operation(5, 5, Operation::Div);

        // Dividing constants 10 / 5
        let result = div(&mut grid, 3, 3);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_div_with_mixed() {
        let mut grid = setup_grid_with_mixed_operation(5, 5, Operation::Div);

        // Dividing cell(1,1) = 6 by const 5
        let result = div(&mut grid, 3, 3);
        assert_eq!(result, Some(1)); // Integer division
    }

    #[test]
    fn test_div_by_zero() {
        let mut grid = setup_test_grid(5, 5);

        // Set up a cell to divide by zero
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Cell(1, 1))),
            Some(Box::new(Value::Const(0))),
            Operation::Div,
        ));
        grid.set_node(3, 3, function_node);

        // Should return None for division by zero
        let result = div(&mut grid, 3, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_div_invalid_cell() {
        let mut grid = setup_grid_with_binary_operation(5, 5, Operation::Div);

        // Make one of the cells invalid
        let invalid_node = grid.get_node(2, 2);
        invalid_node.valid = false;

        // Should return None if one of the cells is invalid
        let result = div(&mut grid, 3, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_slp_normal() {
        let mut grid = setup_test_grid(5, 5);

        // Set up a cell with a sleep operation
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Const(0))), // Sleep for 0 seconds for test speed
            Some(Box::new(Value::Const(0))),
            Operation::Slp,
        ));
        grid.set_node(3, 3, function_node);

        // Should return the sleep time
        let result = slp(&mut grid, 3, 3);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_slp_with_cell() {
        let mut grid = setup_test_grid(5, 5);

        // Set up a cell with a sleep operation
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Cell(0, 0))), // Sleep for value of cell(0,0) = 0
            Some(Box::new(Value::Const(0))),
            Operation::Slp,
        ));
        grid.set_node(3, 3, function_node);

        // Should return the sleep time
        let result = slp(&mut grid, 3, 3);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_slp_invalid_cell() {
        let mut grid = setup_test_grid(5, 5);

        // Set up a cell with a sleep operation
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Cell(0, 0))),
            Some(Box::new(Value::Const(0))),
            Operation::Slp,
        ));
        grid.set_node(3, 3, function_node);

        // Make the cell invalid
        let invalid_node = grid.get_node(0, 0);
        invalid_node.valid = false;

        // Should return None if the cell is invalid
        let result = slp(&mut grid, 3, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_cons_normal() {
        let mut grid = setup_test_grid(5, 5);

        // Set up a cell with a cons operation
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Const(42))),
            Some(Box::new(Value::Const(0))),
            Operation::Cons,
        ));
        grid.set_node(3, 3, function_node);

        // Should return the constant value
        let result = cons(&mut grid, 3, 3);
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_cons_with_cell() {
        let mut grid = setup_test_grid(5, 5);

        // Set up a cell with a cons operation
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Cell(1, 1))), // Get value of cell(1,1) = 6
            Some(Box::new(Value::Const(0))),
            Operation::Cons,
        ));
        grid.set_node(3, 3, function_node);

        // Should return the value of the referenced cell
        let result = cons(&mut grid, 3, 3);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_cons_invalid_cell() {
        let mut grid = setup_test_grid(5, 5);

        // Set up a cell with a cons operation
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Cell(1, 1))),
            Some(Box::new(Value::Const(0))),
            Operation::Cons,
        ));
        grid.set_node(3, 3, function_node);

        // Make the cell invalid
        let invalid_node = grid.get_node(1, 1);
        invalid_node.valid = false;

        // Should return None if the cell is invalid
        let result = cons(&mut grid, 3, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_invalid_function_structures() {
        let mut grid = setup_test_grid(5, 5);

        // Missing first operand
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            None,
            Some(Box::new(Value::Const(5))),
            Operation::Add,
        ));
        grid.set_node(3, 3, function_node);

        assert_eq!(add(&mut grid, 3, 3), None);

        // Missing second operand
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Const(5))),
            None,
            Operation::Add,
        ));
        grid.set_node(3, 3, function_node);

        assert_eq!(add(&mut grid, 3, 3), None);

        // Both operands missing
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(None, None, Operation::Add));
        grid.set_node(3, 3, function_node);

        assert_eq!(add(&mut grid, 3, 3), None);
    }

    #[test]
    fn test_nested_value_types() {
        let mut grid = setup_test_grid(5, 5);

        // Set up a cell with a nested operation
        let mut function_node = Node::new(0);
        function_node.function = Some(Value::Oper(
            Some(Box::new(Value::Oper(
                Some(Box::new(Value::Const(5))),
                Some(Box::new(Value::Const(5))),
                Operation::Add,
            ))),
            Some(Box::new(Value::Const(5))),
            Operation::Add,
        ));
        grid.set_node(3, 3, function_node);

        // Should return None for nested operations
        assert_eq!(add(&mut grid, 3, 3), None);
    }
}
