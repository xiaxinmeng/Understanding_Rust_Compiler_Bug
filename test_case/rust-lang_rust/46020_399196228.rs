rust
#![feature(nll)]
#![allow(warnings)]

fn main() {
    let x = 0;
    x = 1;
    std::mem::replace(&mut x, 2);
}
