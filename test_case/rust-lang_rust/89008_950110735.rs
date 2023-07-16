rust
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use std::future::Future;
use std::marker::PhantomData;

pub trait Stream {
    type Item;
}

trait X {
    type LineStream<'a, Repr>: Stream<Item = Repr> where Self: 'a;
    type LineStreamFut<'a, Repr>: Future<Output = Self::LineStream<'a, Repr>> where Self: 'a;
    fn line_stream<'a, Repr>(&'a self) -> Self::LineStreamFut<'a, Repr>;
}

struct Y;

impl X for Y {
    type LineStream<'a, Repr> = impl Stream<Item = Repr>;
    type LineStreamFut<'a, Repr> = impl Future<Output = Self::LineStream<'a, Repr>>;
    fn line_stream<'a, Repr>(&'a self) -> Self::LineStreamFut<'a, Repr> {
        async { Empty { _phantom: PhantomData } }
    }
}

pub struct Empty<T> {
    _phantom: PhantomData<T>,
}

impl<T> Stream for Empty<T> {
    type Item = T;
}

fn main() {}
