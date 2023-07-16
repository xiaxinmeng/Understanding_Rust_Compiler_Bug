rust
#![feature(async_closure)]
use std::future::Future;

type FA<R> = dyn FnOnce() -> R + 'static;

fn master() -> FA<impl Future<Output = ()>> {
    async move || -> () { () }
}
