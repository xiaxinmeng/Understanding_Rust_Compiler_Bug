rust
use std::iter::Iterator;

pub trait LeapfrogIterator<T, V> : Iterator<Item=(T,V)> {
    fn next_above(&mut self, &T) -> Option<(T, V)>;
}

fn main () {
    let v: Box<LeapfrogIterator<i64, i64>> = unimplemented!();
}
