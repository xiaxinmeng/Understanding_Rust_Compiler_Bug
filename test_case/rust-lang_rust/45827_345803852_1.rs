rust
#![allow(dead_code)]

use std::cell::Cell;

struct Foo<'a, 'b> where 'a: 'b {
    x: Cell<&'a u32>,
    y: Cell<&'b u32>,
}

fn bar<'a, 'b>(x: Cell<&'a u32>, y: Cell<&'b u32>) {
    Foo { x, y };
}

fn main() { }
