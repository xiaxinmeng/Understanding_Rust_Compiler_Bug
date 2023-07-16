rust
async fn foo<'a>(x: &'a str) -> impl Stream<Item = Box<Any>> {}
