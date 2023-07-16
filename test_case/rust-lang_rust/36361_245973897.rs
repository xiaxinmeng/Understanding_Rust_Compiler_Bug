 rust
#![allow(dead_code)]

use E::*;

enum E {
    A,
    B,
    C,
    D,
}

fn main() {
    match A {
    | A
    | B => println!("Give me A | B!"),
    | C
    | D => println!("Why am I here?"),
    }
}
