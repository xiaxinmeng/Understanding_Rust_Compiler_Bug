rust
#![feature(decl_macro)]

macro test() {
    ::std::vec::Vec::new()
}

fn main() {
    let x: Vec<usize> = test!();
}
