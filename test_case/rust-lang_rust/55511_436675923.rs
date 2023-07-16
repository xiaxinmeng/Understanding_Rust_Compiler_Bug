rust
#![feature(nll)]

use std::cell::Cell;

trait Foo<'a> {
    const C: Option<Cell<&'a u32>>;
}

impl<'a, T> Foo<'a> for T {
    const C: Option<Cell<&'a u32>> = None;
}

fn foo<'a>(r: &'a i32) {
    let b = Some(Cell::new(r));
    match b {
        <() as Foo<'static>>::C => { }
        _ => { }
    }
}

fn main() {
}
