 Rust
use std::ops::Add;

fn plus<T>(a: &T, b: &T) -> T where
    for <'a> &'a T: Add<&'a T, Output=T>,
{
    &*a + &*b
}
fn main() {
    println!("1+1={}", plus(&1,&1));
}
