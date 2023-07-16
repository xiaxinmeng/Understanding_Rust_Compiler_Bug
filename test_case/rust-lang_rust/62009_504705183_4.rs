rust
#![feature(async_await)]

fn main() {
    foo();
}

async fn foo() {
    async { let (); }.await
}
