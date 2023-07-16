rust
use std::{mem;

fn main() {
    assert_eq!(4, mem::size_of::<i32>());
}
