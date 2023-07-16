rust
#![feature(type_alias_impl_trait)]
use std::future::Future;

trait Foo {
    type X: Future<Output = ()>;
    fn x() -> Self::X;
}

impl Foo for () {
    type X = impl Future<Output = ()>;
    fn x() -> Self::X {
        async {}
    }
}
