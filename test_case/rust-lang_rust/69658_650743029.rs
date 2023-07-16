rust
// main.rs
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Inner<C> {
    phantom: PhantomData<C>,
}

struct Cloner<C> {
    phantom: PhantomData<C>,
}

impl<C> Cloner<C>
where
    C: Clone,
{
    pub fn call_clone(&self, c: &C) -> C {
        c.clone()
    }
}

fn clear_error<C>() {
    let inner: Inner<C> = Inner {
        phantom: PhantomData,
    };
    let cloner = Cloner {
        phantom: PhantomData,
    };
    cloner.call_clone(&inner);
}

fn confusing_error<C>() {
    let inner = Inner {
        phantom: PhantomData,
    };
    let cloner: Cloner<Inner<C>> = Cloner {
        phantom: PhantomData,
    };
    cloner.call_clone(&inner);
}

fn main() {}

