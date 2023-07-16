rust
#![feature(type_alias_impl_trait)]

use std::future::Future;

type Deferred = impl Future<Output = ()> + 'static;

fn future_out<F>(_: F) -> Deferred {
    async move { }
}
