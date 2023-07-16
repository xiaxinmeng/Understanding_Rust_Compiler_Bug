rust
use std::vec::Vec;

pub enum Foo {
    First(usize),
    Second(usize),
}

pub fn foo(mut v: Vec<Foo>) -> Foo {
    assert!(v.len() == 1);
    v.pop().unwrap()
}
