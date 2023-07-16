rust
#![feature(nll)]

fn foo<T>(x: &mut &mut T) {
    *x = *x;
}

fn main() { }
