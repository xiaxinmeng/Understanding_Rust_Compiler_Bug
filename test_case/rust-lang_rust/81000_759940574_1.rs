rust
// src/main.rs

fn main() {}

pub async fn f() {
    repro::g::<()>().await;
}
