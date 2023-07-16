rust
#![feature(conservative_impl_trait, generators, generator_trait)]

use std::ops::Generator;

trait Tr { }

struct Foo;
impl Tr for Foo { }

struct Bar;
impl Bar {
    fn baz<'a>(&'a mut self) -> impl Tr + 'a {
        Foo
    }
}

fn run<'a>(bar: &'a mut Bar) -> impl Generator<Yield = (), Return = ()> + 'a {
    move || {
        if false {
            yield;
        }

        {
            let _baz = bar.baz();
            if false { yield; }
        }

        {
            bar.baz();
        }
    }
}

fn main() {
}
