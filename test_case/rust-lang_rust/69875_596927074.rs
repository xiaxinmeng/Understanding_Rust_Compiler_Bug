rust
#![feature(or_patterns)]

fn main() {
    if let x @ 0 | x @ (1 | 2) = 0 {}
}
