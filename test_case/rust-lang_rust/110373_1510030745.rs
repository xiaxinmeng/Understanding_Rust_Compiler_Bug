rust
use core::ops::Index;

trait Trait {}
struct Container;

impl<T: Trait> Index<T> for Container {
    type Output = T;

    fn index(&self, idx: T) -> &Self::Output {
        &idx
    }
}

fn main() {
    struct Containee;

    Container[Containee];
}
