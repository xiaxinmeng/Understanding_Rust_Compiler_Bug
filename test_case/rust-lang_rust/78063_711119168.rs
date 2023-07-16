rust
use std::ops::Add;
fn incr<T:Add<i32>>(x: T) -> T::Output {
    x + 1 // this is fine
}

fn main() {
    incr(()); //~ ERROR cannot add `i32` to `()`
}
