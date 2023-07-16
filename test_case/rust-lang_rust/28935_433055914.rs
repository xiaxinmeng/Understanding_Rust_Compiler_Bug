rust
use std::cell::RefCell;

fn f(v: Vec<RefCell<u8>>) {
    let _t = &mut *v[0].borrow_mut();
}

fn main() {}
