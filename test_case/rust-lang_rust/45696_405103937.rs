rust
#![feature(nll)]

fn foo(x: Box<&mut i32>) -> &mut i32 {
    *x
}

fn main() {}
