rust
#![feature(existential_type)]

trait Future {
}

struct AndThen<F>(F);

impl<F> Future for AndThen<F> {
}

struct Foo<'a> {
    x: &'a mut (),
}

existential type F: Future;

impl<'a> Foo<'a> {
    fn reply(&mut self) -> F {
        AndThen(|| ())
    }
}
