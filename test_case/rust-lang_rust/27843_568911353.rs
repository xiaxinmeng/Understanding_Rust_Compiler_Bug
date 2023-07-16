rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

trait Baz { fn get(&self) -> i32; }
impl Baz for i32 { fn get(&self) -> i32 { *self } }

fn main() {
    let a: Rc<RefCell<i32>> = Rc::new(RefCell::new(42));
    let c1: Weak<RefCell<dyn Baz>> = Rc::downgrade(&a);             // Doesn't work
    let c2: Weak<RefCell<dyn Baz>> = Rc::downgrade(&a) as Weak<_>;  // works
}
