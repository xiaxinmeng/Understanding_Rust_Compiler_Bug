rust
#![feature(async_await)]

#[derive(Default)]
struct Foo {
    
}

impl Drop for Foo {
    fn drop(&mut self) { }
}

async fn bar() {
}

pub async fn foo() {
    Foo::default();
    bar().await;
}
