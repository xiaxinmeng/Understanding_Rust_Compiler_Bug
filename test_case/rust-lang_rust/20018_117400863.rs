 rust
use std::ops::Add;

trait Vector<T: Add<T>> {
    fn map_in_place(&mut self, f: (Fn(T) -> T));
}

impl<T: Add<T>, V: Vector<T>> Add<V> for V {
    fn add(&self, rhs: &V) -> V {
        unimplemented!();
    }
}

fn main() { }
