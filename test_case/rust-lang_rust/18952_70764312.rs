 rust
#![feature(unboxed_closures)]
use std::ops::Fn;
struct Foo;

impl Fn(i32, i32) for Foo {
    extern "rust-call" fn call(&self, args: (i32, i32)) {
        println!("{:?}", args);
    }
}

impl Fn(i32, i32) -> i32 for Foo {
    extern "rust-call" fn call(&self, args: (i32, i32)) -> i32 {
        println!("{:?}", args);
        0
    }
}

fn main() {
    let foo = Foo;
    let x: i32 = foo(1, 1);
}
