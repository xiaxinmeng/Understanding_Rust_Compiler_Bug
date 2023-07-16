rust
#![feature(type_alias_impl_trait)]

use std::future::Future;

type X<T> = Box<dyn Future<Output = T>>;

trait Service {
    type Something;

    fn stream<'a, 'b>(&self) -> X<Self::Something>
    where
        'a: 'b;
}

impl Service for () {
    type Something = impl Sized;

    fn stream<'a, 'b>(&self) -> X<Self::Something>
    where
        'a: 'b,
    {
        Box::new(async move { std::iter::from_fn(|| Some(())) })
    }
}
fn main() {}
