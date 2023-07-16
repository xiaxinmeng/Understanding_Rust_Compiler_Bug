rust
extern crate futures;

use futures::{Future, Stream, Sink};
use futures::stream::iter;

fn foo<T>() -> T {
    loop {}
}

fn main() {
    iter(foo::<std::iter::Cloned<std::slice::Iter<i32>>>().map(Ok))
        .forward(
            foo::<Box<Sink<SinkItem = i32, SinkError = ()>>>()
        )
        .join(
            foo::<Box<Stream<Item = i32, Error = ()>>>()
                .for_each(|_| Ok(()))
        );
}
