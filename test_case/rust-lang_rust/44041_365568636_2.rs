rust
use std::cmp::Ordering::*;
pub fn mytest(x: i32, y: i32, i: usize) -> usize {
    let x = match x.cmp(&y) {
        Equal => ::std::process::exit(0),
        Less => 0,
        Greater => 1,
    };

    2 * (i + 1) - x
}
