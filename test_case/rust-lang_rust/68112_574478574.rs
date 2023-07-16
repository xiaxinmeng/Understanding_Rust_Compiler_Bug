rust

#![feature(optin_builtin_traits)]
// edition:2018
struct Foo;

impl !Send for Foo {}

fn is_send<T: Send>(t: T) { }

async fn bar() {
    let x = Foo;
    baz().await;
}

async fn baz() { }

fn main() {
    is_send(bar());
}
