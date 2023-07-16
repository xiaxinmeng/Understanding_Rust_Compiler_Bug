rust
#![feature(specialization)]
use std::marker::PhantomData;

trait Trait {
    type A;
    type B;

    fn foo(&self, a: Self::A, b: Self::B);
}

struct Foo<A, B> {
    a: PhantomData<A>,
    b: PhantomData<B>,
}

impl<A, B> Foo<A, B> {
    fn new() -> Self {
        Foo {
            a: PhantomData,
            b: PhantomData,
        }
    }
}

impl<A, B> Trait for Foo<A, B> {
    type A = A;
    type B = B;
    default fn foo(&self, _: A, _: B) {
        println!("default impl");
    }
}

// Specialized
impl<A, B: Eq> Trait for Foo<A, B> {
    fn foo(&self, _: A, _: B) {
        println!("specialized");
    }
}

fn main() {
    let a = "a";
    let b = "b";

    let f = Foo::new(); // Need to specify concrete type here to compile
    f.foo(a, b);
}
