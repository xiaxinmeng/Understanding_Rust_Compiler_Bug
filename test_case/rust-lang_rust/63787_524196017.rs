rust
use std::cell::*;

pub fn break_it(rc: &RefCell<Option<Box<i32>>>, r: Ref<'_, i32>) {
    drop(r);
    *rc.borrow_mut() = None;
}

pub fn main() {
    let rc = RefCell::new(Some(Box::new(0)));
    break_it(&rc, Ref::map(rc.borrow(), |x| &**x.as_ref().unwrap()))
}
