 rust
use std::cell::{Cell, RefCell};
struct A<T: Fn()> {
    x: RefCell<Option<T>>,
    b: Cell<i32>,
}

fn main() {
    let mut p = A{x: RefCell::new(None), b: Cell::new(4i32)};
    let q = || p.b.set(5i32);
    *(p.x.borrow_mut()) = Some(q);
}
