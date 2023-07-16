rust
#![feature(generic_associated_types)]

use std::marker;

trait Trait {
    type Gat<'this>
    where
        Self: 'this;

    fn function<F>(self, _: F)
    where
        F: FnOnce(Self::Gat<'_>);

    fn function2(self);
}

struct Struct<T>(marker::PhantomData<T>);

impl<T> Trait for Struct<T> {
    type Gat<'this> = &'this mut T where Self: 'this;

    fn function<F>(self, callback: F)
    where
        F: FnOnce(&mut T),
    {
    }

    fn function2(self) {
        self.function(|value| {})
    }
}
