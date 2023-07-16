Rust
#![feature(generic_const_exprs)]
use std::marker::PhantomData;

pub trait Foo {
    type Quiz;
    fn bar() -> Self::Quiz;
}

#[derive(Clone, Debug)]
struct Corge<const K: usize> {}

impl<const K: usize> Foo for Corge<K>
where
    [(); K + 0]:,
{
    type Quiz = [u16; K + 0];
    fn bar() -> Self::Quiz {
        [0u16; K + 0]
    }
}

struct Gork<R: Foo> {
    t: PhantomData<R>,
}

impl<R: Foo> Gork<R> {
    fn baz(self) -> Vec<R::Quiz> {
        vec![R::bar()]
    }
    //fn baz(self) -> [R::Quiz; 1] {
    //    [R::bar()]
    //}
}

fn main() {
    let x = Gork::<Corge<3>> { t: PhantomData::default() };
    let _ = x.baz();
}
