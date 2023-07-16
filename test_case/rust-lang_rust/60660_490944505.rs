
#![feature(async_await, await_macro)]

async fn foo() {}

async fn bar() {
    std::r#await!(foo())
}
