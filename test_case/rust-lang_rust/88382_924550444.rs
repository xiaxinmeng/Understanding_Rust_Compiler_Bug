rust
#![feature(generic_associated_types)]

use std::marker::PhantomData;

trait Iterable {
    type Iterator<'a>;
    fn iter(&self) -> Self::Iterator<'_>;
}

struct SomeImplementation();

impl Iterable for SomeImplementation {
    type Iterator<'a> = std::iter::Empty<usize>;
    fn iter(&self) -> Self::Iterator<'_> {
        std::iter::empty()
    }
}

struct DoSomething<I: Iterable, F: Fn(&mut I::Iterator<'_>)> {
    f: F,
    _phantom: PhantomData<I>,
}

impl<I: Iterable, F: Fn(&mut I::Iterator<'_>)> DoSomething<I, F> {
    fn really_do_something(&self, i: I) {
        (self.f)(&mut i.iter());
    }
}

fn main() {
    let do_something = DoSomething::<SomeImplementation, _> {
        f: |_| (),
        _phantom: PhantomData,
    };
    do_something.really_do_something(SomeImplementation());
}

fn test<'a, I: Iterable>(_: &mut I::Iterator<'a>) {}
