rust
#![feature(untagged_unions)]
use std::cell::Cell;

pub union U { i: u32, c: Cell<u32> }

pub const CELL: U = {
    U { i: 0 }
};

fn main() {
    let cell: &'static U = &CELL;
    unsafe { (cell.c).set(1) };
}
