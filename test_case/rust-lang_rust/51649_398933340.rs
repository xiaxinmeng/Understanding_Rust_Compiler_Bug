rust
#![feature(nll)]

use std::ops::Deref;

struct A<T>(T);

impl<T> Deref for A<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}
