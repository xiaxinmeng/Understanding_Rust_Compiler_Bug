rust
#![feature(generic_const_exprs)]
pub struct Foo<T, const H: T>(T)
where
    [(); {1}]:;
