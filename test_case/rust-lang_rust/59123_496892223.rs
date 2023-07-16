rust
#![feature(generators, generator_trait)]

use std::ops::Generator;

struct Foo([u8; 1024]);

fn simple() -> impl Generator<Yield = (), Return = ()> {
    static || {
        let first = Foo([0; 1024]);
        let _second = first;
        yield;
    }
}

fn complex() -> impl Generator<Yield = (), Return = ()> {
    static || {
        fn foo(_: &mut Foo) {}
        
        let mut first = Foo([0; 1024]);
        foo(&mut first);
        let mut second = first;
        foo(&mut second);
        let mut third = second;
        foo(&mut third);
        let mut _fourth = third;
        
        yield;
    }
}

fn main() {
    dbg!(std::mem::size_of_val(&simple()));
    dbg!(std::mem::size_of_val(&complex()));
}
