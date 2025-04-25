use project::extension::common::*;

#[test]
pub fn tesst() {
    let mut val = Value::Const(5);
    let my_val = val.row();
    assert_eq!(my_val, 0);
    let my_val = val.col();
    assert_eq!(my_val, 0);
    let mut my_val = val.assign_row(2);
    assert_eq!(my_val, ());
    let mut my_val = val.assign_col(3);
    assert_eq!(my_val, ());
}
