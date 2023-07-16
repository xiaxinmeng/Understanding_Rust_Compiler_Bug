rust
#![feature(nll)]
#![allow(warnings)]

fn main() {
    let x = 42;
    let y: &'static _ = &|| { let x = x; x };
}
