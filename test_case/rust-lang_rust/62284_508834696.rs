rust
#![feature(async_await)]

async fn foo(n: usize) {
    if n > 0 {
        Box::new(foo(n - 1)).await;
    }
}

fn is_send<T: Send>(t: T) { drop(t); }

fn main() {
    is_send(foo());
}
