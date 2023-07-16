rust
#![feature(conservative_impl_trait, generators, generator_trait)]

extern crate futures;

use std::ops::{ Generator, GeneratorState };
use futures::{ Async, future, Future };

struct Foo {
    i: u8,
}

impl Foo {
    fn i<'a>(&'a mut self) -> impl Future<Item = u8, Error = ()> + 'a {
        future::ok(self.i)
    }
}

fn run<'a>(foo: &'a mut Foo) -> impl Generator<Yield = (), Return = Result<u8, ()>> + 'a {
    move || {
        if false {
            yield
        }

        let _ = {
            let mut f = foo.i();
            loop {
                let poll = f.poll();
                match poll {
                    Ok(Async::Ready(i)) => break i,
                    Ok(Async::NotReady) => yield,
                    _ => unreachable!(),
                }
            }
        };

        {
            let mut f = foo.i();
            let poll = f.poll();
            match poll {
                Ok(Async::Ready(i)) => Ok(i),
                _ => unreachable!(),
            }
        }
    }
}

fn main() {
    let mut foo = Foo { i: 0 };
    let mut g = run(&mut foo);
    match g.resume() {
        GeneratorState::Complete(Ok(i)) => println!("{}", i),
        _ => unreachable!(),
    }
}

