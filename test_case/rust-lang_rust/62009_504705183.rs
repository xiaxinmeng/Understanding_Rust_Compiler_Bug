rust
#![feature(async_await)]

fn main() {
    async { let (); }.await
}
