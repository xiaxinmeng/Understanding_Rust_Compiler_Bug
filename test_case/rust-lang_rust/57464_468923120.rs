rust
use std::{iter, slice};

struct A<T>(T);

unsafe impl<'a, 'b, F, G> Send for A<iter::Map<iter::Map<slice::Iter<'_, usize>, F>, G>>
where
    F: FnOnce(&'a usize) -> &'b usize
{}

fn iter<'a>(data: &'a [usize]) -> impl Sized + 'a {
    A(data.iter()
        .map(
            |x| x // FnMut(&'a usize) -> &'(ReScope) usize
        )
        .map(
            |x| *x // FnMut(&'(ReScope) usize) -> usize
        ))
}

fn main() {
    let x: Box<dyn Send> = Box::new(iter(&[1, 2, 3]));
}
