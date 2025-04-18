/// Represents a value that can appear in a spreadsheet.
#[derive(Debug)]
pub enum Value {
    Cell(i32, i32),
    Const(isize),
    Oper(Box<Value>, Box<Value>, Operation), //value1 and value2, and the operation or command, respectively
}

#[derive(Debug)]
pub enum Operation {
    Cons,
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
    Avg,
    Sum,
    Std,
    Slp,
    EnableOutput,
    DisableOutput,
    ScrollTo,
}
