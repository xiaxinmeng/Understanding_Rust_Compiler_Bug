Rust
use std::fmt::Debug;

trait Left<T> {}
trait Right<T> {}

impl<T, U: Default + Debug> Left<U> for T {}
impl<T, U: Default + Debug> Right<U> for T {}

// replacing the order of `T: Left<U>` & `T: Right<U>` changes result
fn movable<T, U>() where T: Left<U>, T: Right<U>, U: Default + Debug {
    println!("{:?}", U::default())
}

fn try_it<T: Default + Debug>() where T: Left<bool>, T: Right<()> {
    movable::<T, _>();
}

fn main() {
    try_it::<u8>() // the type here is irrelevant
}
