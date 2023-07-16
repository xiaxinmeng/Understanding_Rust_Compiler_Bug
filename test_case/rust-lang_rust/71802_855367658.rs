rust
use std::cell::Cell;
const X: Option<Cell<i32>> = None;
fn main() {
    let x: &'static _ = &X;
}
