rust
// lib.rs
#![feature(const_generics, const_evaluatable_checked)]
#![allow(incomplete_features)]

pub struct Foo<const N: usize>([(); N + 1])
where
    [(); N + 1]: ;

// main.rs
#![feature(const_generics, const_evaluatable_checked)]
#![allow(incomplete_features)]

use ice::Foo;

fn new<U>(a: U) -> U {
    a
}

fn foo<const N: usize>(bar: &mut Foo<N>)
where
    [(); N + 1]: ,
{
    *bar = new(loop {});
}

fn main() {}
