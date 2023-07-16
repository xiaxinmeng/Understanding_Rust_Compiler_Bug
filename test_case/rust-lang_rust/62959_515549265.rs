rust
#![feature(array_value_iter)]
use std::array::IntoIter;

fn main() {
    let arr = [1, 2, 3];
    let it = IntoIter::new(arr);
}
