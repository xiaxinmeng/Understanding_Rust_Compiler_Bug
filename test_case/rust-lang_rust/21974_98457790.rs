 rust
use std::ops::Add;

fn plus<T>(a: &T, b: &T) -> T where
    for <'a> &'a T: Add<&'a T, Output=T>,
{
    a + b // error: cannot infer an appropriate lifetime for automatic coercion due to conflicting requirements
}
fn main() {}
