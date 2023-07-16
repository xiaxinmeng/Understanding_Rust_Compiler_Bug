rust
#![feature(pin_as_inner_ref)]

use std::{cell::Cell, marker::PhantomPinned, pin::Pin, sync::Arc};

#[derive(Default)]
struct Thing {
    my_own_address: Cell<Option<*const Thing>>,
    _pinned: PhantomPinned,
}

impl Thing {
    fn x(self: Pin<&Self>) {
        self.my_own_address.set(Some(&*self));
    }
}

impl Drop for Thing {
    fn drop(&mut self) {
        let addr = self.my_own_address.get();
        assert!(addr == None || addr == Some(&*self), "pinned object moved!");
    }
}

fn main() {
    let thing = Arc::pin(Thing::default());
    thing.as_ref().x();
    let clone = thing.as_inner_ref().clone();
    drop(thing);
    drop(Arc::try_unwrap(clone));
}
