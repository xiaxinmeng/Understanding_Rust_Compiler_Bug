`rust
#![feature(generators)]

struct Foo();

impl Drop for Foo {
    fn drop(&mut self) {}
}

fn take<T>(_x: T) {}

fn main() {
    let _gen = || {
        let a = Foo();
        yield;
        take(a);
    };
} 
