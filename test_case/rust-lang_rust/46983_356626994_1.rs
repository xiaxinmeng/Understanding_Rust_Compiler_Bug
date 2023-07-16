rust
#![feature(nll)]

fn foo<'a>(x: &u32) -> &'a u32 {
    &*x
}

fn main() {}
