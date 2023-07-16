rust
#![feature(nll)]

use std::ops::Index;

pub struct Test<T> {
    a: T,
}

impl<T> Index<usize> for Test<T> {
    type Output = T;

    fn index(&self, _index: usize) -> &Self::Output {
        &self.a
    }
}

fn main() {}
