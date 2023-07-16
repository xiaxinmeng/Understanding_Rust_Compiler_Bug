rust
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

trait Trait<T> {
    fn fnc<const N: usize = "">(&self) {}
}

fn main() {}
