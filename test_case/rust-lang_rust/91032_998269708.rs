rust
#![feature(negative_impls)]

fn main() {
    gimme_send(foo());
}

fn gimme_send<T: Send>(t: T) {
    drop(t);
}

struct NotSend {}

impl Drop for NotSend {
    fn drop(&mut self) {}
}

impl !Send for NotSend {}

async fn foo() {
    let mut x = (NotSend {},);
    drop(x.0);
    x.0 = NotSend {};
    bar().await;
}

async fn bar() {}
