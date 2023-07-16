rust
#![feature(non_lifetime_binders, generic_const_exprs)]

fn foo() -> usize
where
    for<T> [i32; 0: T]:,
{}

fn main() {}
