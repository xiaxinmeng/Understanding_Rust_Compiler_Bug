rust
use std::cell::RefCell;
use std::rc::Rc;

trait Trait: 'static {}

struct Store<C> {
    inner: Rc<RefCell<Option<C>>>,
}

fn main() {
    let store = Store::<Box<for<'a> fn(&(dyn Trait + 'a))>> {
        inner: Default::default(),
    };
}
