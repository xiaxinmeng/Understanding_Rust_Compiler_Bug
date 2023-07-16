rust
#![feature(ptr_metadata)]

use core::marker::PhantomData;
use core::ops::Deref;
use core::ptr::{DynMetadata, Pointee};

struct Foo<T> {
    bar: PhantomData<T>,
}

impl<T> Deref for Foo<T>
where
    T: Pointee<Metadata = DynMetadata<T>> + 'static,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.some_method()
    }
}
