rust
#![feature(const_fn)]

#[derive(PartialEq, Eq)]
struct S;

const fn new() -> S { S }

const ES: S = new();

fn main() {
    match S {
        ES => {}
    }
}
