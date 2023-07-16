
#![feature(generators, conservative_impl_trait, generator_trait)]

use std::ops::{Generator, GeneratorState};

fn foo() -> Box<Generator<Yield = (), Return = ()>> {
    Box::new(|| {
        let mut gen = foo();
        
        let mut r = gen.resume();
        while let GeneratorState::Yielded(v) = r {
            yield v;
            r = gen.resume();
        }
    })
}
