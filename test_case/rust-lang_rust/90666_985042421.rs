rust
use std::sync::{Arc, Weak};

struct Foo {
    me: Weak<Foo>,
}

impl Foo {
    /// Construct a reference counted Foo.
    fn new() -> Arc<Self> {
        Arc::new_cyclic(|me| Foo {
            me: me.clone(),
        })
    }

    /// Return a reference counted pointer to Self.
    fn me(&self) -> Arc<Self> {
        self.me.upgrade()
    }
}
