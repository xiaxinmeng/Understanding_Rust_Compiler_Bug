rust
#![feature(closure_lifetime_binder)]
#![feature(trait_alias)]

use core::future::Ready;
use core::future::ready;
use core::future::Future;

// Don't read too much into the implementation of `S`
struct S<'a>(&'a ());

trait AsyncF<'a, F> = Fn(S<'a>) -> F where F: Future<Output = S<'a>>;
async fn asyncF<F>(f: impl for<'a> AsyncF<'a, F>) {
    let a = ();
    f(S(&a)).await;
}

fn main() {
    // We can't seem to do this because for<> does not seem to extend to the
    // argument.
    asyncF(for<'a> |x: S<'a>| -> Ready<S<'a>> {ready(x)});

    
    // Ideally we would like this to do anything useful with for<> and async.
    // asyncF(for<'a> async move |x: &'a ()| -> &'a () {ready(x)});
}
