use std::cell::RefCell;
use std::rc::Rc;

pub struct A{
    pub val: Option<i32>
}

fn main() {
    let a = Rc::new(RefCell::new(A {val: Some(123)}));
    if let Some(val) = a.borrow().val {
        print!("val in a is {}", val);
    }
}
