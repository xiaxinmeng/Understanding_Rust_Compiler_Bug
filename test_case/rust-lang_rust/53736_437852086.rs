rust
#![feature(async_await, futures_api, generators)]

// A trait with 'static bound
trait Trait: 'static {}

// The actual reproducer, requires that Trait is 'static and a trait
// object to it is captured in a closure for successful reproduction.
async fn foo(b: Box<Trait + 'static>) -> () {
    let _bar = move || { b; () };
    yield
}

pub fn main() {}
