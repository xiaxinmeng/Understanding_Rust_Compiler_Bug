rust
#![allow(unused)]

enum O<T> { Some(T), None, }

use O::*;

fn main() {
    let x = Some(1);
    let Some(y) = x;
}
