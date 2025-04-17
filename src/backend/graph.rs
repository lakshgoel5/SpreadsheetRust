//topo sort
//node
//adding
//breaking
//cycle
//reset

#[derive(Debug)]
pub struct Node {
    pub dependents: Vec<Value::Cell>,
    pub node_value: i32,
    pub function: Value::Oper,
    pub visited: bool,
    pub valid: bool,
}