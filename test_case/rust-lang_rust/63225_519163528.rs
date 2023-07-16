rust
#![feature(async_await)]

struct Foo<'a>(&'a u8);

impl Foo<'_> {
    async fn bar() {}
}
