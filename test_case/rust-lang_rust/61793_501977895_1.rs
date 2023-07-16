rust
#![feature(async_await)]
#![allow(unused)]

async fn foo<F>(_: &(), _: F) {}

fn main() {
    foo(&(), || {});
}
