rust
#![feature(async_await)]

async fn foo(x: Box<dyn Send>) {
    async fn bar() {}
    let x = &x;
    bar().await;
}

fn assert_send<T: Send>(_: T) {}

fn main() {
    assert_send(foo(Box::new(5)));
}
