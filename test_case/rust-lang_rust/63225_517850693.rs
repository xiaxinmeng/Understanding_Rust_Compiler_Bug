rust
#![feature(async_await)]

struct Foo<'a>(&'a ());

impl<'a> Foo<'a> {
    fn test() {
        async fn test<'a>() {}
    }
}
