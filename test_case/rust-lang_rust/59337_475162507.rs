rust
#![feature(async_await, futures_api, trait_alias, unboxed_closures)]

use std::future::Future;

trait Foo<'a> = FnOnce<(&'a u8,)> where <Self as FnOnce<(&'a u8,)>>::Output: Future<Output = u8> + 'a;

fn foo<F>(f: F) where F: for<'a> Foo<'a> {
    let bar = 5;
    f(&bar);
}

fn main() {
    foo(async move | f: &u8 | { *f });

    foo({ async fn baz(f: &u8) -> u8 { *f } baz });
}
