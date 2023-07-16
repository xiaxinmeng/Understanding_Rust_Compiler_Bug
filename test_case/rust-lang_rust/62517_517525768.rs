rust
#![feature(async_await)]

trait Object {}

trait Alpha<Param> {}

async fn foo<'a>(_: &'a ()) -> impl Alpha<dyn Object> {}
