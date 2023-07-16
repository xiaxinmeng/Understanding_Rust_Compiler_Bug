 rust
#![feature(generic_associated_types)]

use std::marker::PhantomData;

pub trait Iterable {
    type Iterator<'a>;
    fn iter(&self) -> Self::Iterator<'_>;
}

pub struct SomeImplementation();

impl Iterable for SomeImplementation {
    type Iterator<'a> = std::iter::Empty<usize>;
    fn iter(&self) -> Self::Iterator<'_> {
        std::iter::empty()
    }
}

pub struct DoSomething<I: Iterable, F: Fn(&mut I::Iterator<'_>)> {
    pub f: F,
    pub _phantom: PhantomData<I>,
}

impl<I: Iterable, F: Fn(&mut I::Iterator<'_>)> DoSomething<I, F> {
    pub fn really_do_something(&self, i: &mut I) {
        (self.f)(&mut i.iter());
    }
}

fn main() {
    let foo = 42;
    let do_something = DoSomething::<SomeImplementation, _> {
        f: |i| {
            i.next();
        },
        _phantom: PhantomData,
    };
    do_something.really_do_something(&mut SomeImplementation());
    do_some_more(SomeImplementation(), do_something);
}

fn do_some_more<I: Iterable, F: Fn(&mut I::Iterator<'_>)>(
    iterable: I,
    callback: DoSomething<I, F>,
) {
    callback.really_do_something(&mut iterable);
}
