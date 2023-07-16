rust
#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};

fn foo() -> impl Generator<Yield = (), Return = ()> {
    || {
        let mut gen = Box::pin(foo());
        let mut r = gen.as_mut().resume(());
        while let GeneratorState::Yielded(v) = r {
            yield v;
            r = gen.as_mut().resume(());
        }
    }
}

fn main() {
    foo();
}
