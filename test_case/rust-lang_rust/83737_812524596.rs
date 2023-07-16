rust
// edition:2018
// compile-flags: --crate-type rlib
// build-pass

use core::future::Future;

async fn handle<F>(slf: &F)
where
    F: Fn(&()) -> &mut (dyn Future<Output = ()> + Unpin),
{
    (slf)(&()).await;
}
