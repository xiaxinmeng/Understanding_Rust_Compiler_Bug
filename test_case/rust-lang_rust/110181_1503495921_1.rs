rs
// file : main.rs
#![feature(generators, generator_trait)]
use core::{
    ops::{Generator, GeneratorState},
    pin::pin,
};
fn generator_fn() -> impl Generator<Yield = usize, Return = ()> /* not Unpin */ {
    // Allow generator to be self-referential (not `Unpin`)
    // vvvvvv        so that locals can cross yield points.
    static || {
        let foo = String::from("foo");
        let foo_ref = &foo; // ------+
        yield 0; // | <- crosses yield point!
        println!("{foo_ref}"); // <--+
        yield foo.len();
    }
}
fn main() {
    let mut generator = pin!(generator_fn());
    match generator.as_mut().resume(()) {
        GeneratorState::Yielded(0) => {}
        _ => unreachable!(),
    }
    match generator.as_mut().resume(()) {
        GeneratorState::Yielded(3) => {}
        _ => unreachable!(),
    }
    match generator.resume(()) {
        GeneratorState::Yielded(_) => unreachable!(),
        GeneratorState::Complete(()) => {}
    }
}
