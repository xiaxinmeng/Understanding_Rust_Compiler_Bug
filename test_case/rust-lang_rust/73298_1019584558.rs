Rust
#![feature(generic_const_exprs)]
use std::marker::PhantomData;
use std::mem::MaybeUninit;

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
    type Quiz = [u8; K + 0];
    fn bar() -> Self::Quiz {
        [0u8; K + 0]
    }
}

struct Gork<R: Foo> {
    t: PhantomData<R>,
}

impl<R: Foo> Gork<R>
where
    R::Quiz: Copy,
{
    fn baz() {
        unsafe { MaybeUninit::<[R::Quiz; 1]>::uninit().assume_init() };
        //[R::bar(); 1];
    }
}

fn main() {
    Gork::<Corge<3>>::baz()
}
