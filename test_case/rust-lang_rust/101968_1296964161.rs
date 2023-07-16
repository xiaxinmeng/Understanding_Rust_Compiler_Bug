rust
#![feature(impl_trait_in_fn_trait_return)]
use core::future::Future;

fn future_closure() -> impl Fn() -> impl Future<Output = bool> {
    let f = || async { true };
    f
}
