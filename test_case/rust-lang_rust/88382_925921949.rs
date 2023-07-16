rust
#![feature(generic_associated_types)]

use std::marker::PhantomData;

trait Iterable {
    type Iterator<'a>
    where
        Self: 'a;
    fn iter(&self) -> Self::Iterator<'_>;
}

struct SomeImplementation<'a> {
    data: &'a [usize],
}

impl<'a> SomeImplementation<'a> {
    fn new(data: &'a [usize]) -> Self {
        SomeImplementation { data }
    }
}

impl<'d> Iterable for SomeImplementation<'d> {
    type Iterator<'a>
    where
        Self: 'a,
    = std::slice::Iter<'a, usize>;
    fn iter(&self) -> Self::Iterator<'_> {
        self.data.iter()
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
    let data = Vec::<usize>::new();
    DoSomething {
        f: |_| (),
        _phantom: PhantomData,
    }
    .really_do_something(SomeImplementation::new(&data));
}
