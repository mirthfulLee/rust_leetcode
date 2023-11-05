// similar to the error in: https://users.rust-lang.org/t/borrowed-value-life-time-if-let-and-refcell/74025
use std::cell::RefCell;
use std::rc::Rc;

pub struct A{
    pub val: Option<i32>
}

fn main() {
    let a = Rc::new(RefCell::new(A {val: Some(123)}));
    // ERROR condition: if let is the last sentence in the block & it contain `borrow`
    // ATTENTION: if let is equivalent to match, and if it is the last sentence in the block, 
    //      it is actually an expression even it return nothing
    // EXPLAINATION: it will create an temporary for `a.borrow().xxx` and its lifetime equals 
    //      to the return value, which is longger than local var - `a`
    // SOLUTION: add ';'
    if let Some(val) = a.borrow().val {
        print!("val in a is {}", val);
    }  // add ';' here
}