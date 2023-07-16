rust
#![feature(const_panic)]
#![allow(const_err)]

const VOID: ! = panic!();

fn main() {
    let _ = VOID;
}
