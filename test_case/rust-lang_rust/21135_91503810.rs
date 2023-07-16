 rust
#![feature(unsafe_destructor)]
#![allow(dead_code)]

use std::marker::PhantomData;

struct Foo<T> ( PhantomData<T> );

struct Bar<T,U> ( PhantomData<(T, U)> );

#[unsafe_destructor]
impl <T> Drop for Foo<T> {
    fn drop(&mut self) {
    }
}

fn main() {
    let x: Foo<Bar<u8,u8>>;
}
