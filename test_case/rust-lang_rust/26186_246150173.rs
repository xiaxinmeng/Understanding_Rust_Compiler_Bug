 rust
use std::cell::RefCell;
use std::rc::Rc;

struct Foo(Rc<RefCell<FnMut()>>);

fn main() {
    let a = Foo(Rc::new(RefCell::new(||{
        println!("Hello, world!");
    })));
    a.0.borrow_mut()();
}
