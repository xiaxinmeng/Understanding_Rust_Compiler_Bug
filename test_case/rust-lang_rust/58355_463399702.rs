rust
use std::cell::RefCell;

pub fn foo(callback: &'static Fn() -> ToString) {
    let x: RefCell<Option<Box<Fn() -> ToString>>> = RefCell::new(None);
    *x.borrow_mut() = Some(Box::new(callback));
}
