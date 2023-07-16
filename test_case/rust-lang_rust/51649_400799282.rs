rust
#![feature(nll)]
use std::ops::Deref;
pub struct Foo<T>(T);
impl<T> Deref for Foo<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
