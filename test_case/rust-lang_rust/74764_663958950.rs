rust
use std::ops::Index;

fn main() {
    let vec = vec![1];
    let handler: &dyn Index<usize, Output = i32> = &vec;
    handler.index(2);
}
