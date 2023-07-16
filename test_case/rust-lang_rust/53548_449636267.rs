rust
#![feature(generators)]

use std::cell::RefCell;
use std::rc::Rc;

trait Trait: 'static {}

struct Store<C> {
    inner: Rc<RefCell<Option<C>>>,
}

#[test]
fn test_compose() {
    Box::new(static move || {
        let store = Store::<Box<dyn Trait>> {
            inner: Default::default(),
        };
        yield ();
    });
}
