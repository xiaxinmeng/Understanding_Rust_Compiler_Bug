Rust
#![feature(futures_api)]
#![feature(async_await)]
#![feature(await_macro)]

use std::future::Future;

async fn rec() -> impl Future { await!(rec()) }

fn main() {
    
    rec();
}
