rust
#![feature(async_await)]
#![feature(futures_api)]

pub async fn asynchronous(foo: &str) -> String {
    format!("{}{}", foo, bar)
}

pub fn not_asynchronous(foo: &str) -> String {
    format!("{}{}", foo, bar)
}
