let result = parser::validate(&cmd, &rows, &columns);

if let Some((cell, Some(value))) = result {
    match value {
        Value::Oper(left_operand, right_operand, operation) => {
            match operation {
                Operation::Add => {
                    // Handle addition operation
                    println!("Addition operation");
                    // Do something with left_operand and right_operand
                }
                Operation::Sub => {
                    // Handle subtraction operation
                    println!("Subtraction operation");
                }
                Operation::Mul => {
                    // Handle multiplication operation
                    println!("Multiplication operation");
                }
                Operation::Div => {
                    // Handle division operation
                    println!("Division operation");
                }
                Operation::Sum => {
                    // Handle SUM range operation
                    println!("SUM operation");
                }
                Operation::Avg => {
                    // Handle AVG range operation
                    println!("AVG operation");
                }
                Operation::Std => {
                    // Handle STDEV range operation
                    println!("STDEV operation");
                }
                Operation::Min => {
                    // Handle MIN range operation
                    println!("MIN operation");
                }
                Operation::Max => {
                    // Handle MAX range operation
                    println!("MAX operation");
                }
                Operation::Slp => {
                    // Handle SLEEP operation
                    println!("SLEEP operation");
                }
                Operation::Cons => {
                    // Handle constant assignment or cell reference
                    println!("Constant or cell reference");
                }
            }
        }
        Value::Cell(col, row) => {
            // Handle the case where it's just a cell reference
            println!("Cell reference: Column {}, Row {}", col, row);
        }
        Value::Const(val) => {
            // Handle the case where it's just a constant
            println!("Constant value: {}", val);
        }
    }
} else {
    println!("Invalid command or parsing error");
}