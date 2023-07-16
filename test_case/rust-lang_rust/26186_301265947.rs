rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let a: Rc<RefCell<FnMut()>> = Rc::new(RefCell::new(||{}));
    a.borrow_mut()();
}
