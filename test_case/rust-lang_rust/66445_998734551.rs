rust
use futures::stream::*;

async fn same_error_twice() -> Vec<u8> {
    once(async { 0.0 }).collect().await
}

fn main() {}
