rust
#![feature(generators, generator_trait)]

use std::ops::Generator;

trait Foo: Send { }

impl Foo for u32 { }

fn bar() -> impl Generator + Send {
    || {
        let bar: Box<dyn Foo + 'static> = Box::new(5);
        yield;
        drop(bar)
    }
}

fn main() {}
