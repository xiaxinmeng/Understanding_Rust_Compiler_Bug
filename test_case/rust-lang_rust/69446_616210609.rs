rust
use std::future::Future;
async fn x<T: Future<Output = ()>>(fun: impl FnMut() -> T) {}
async fn main() {
    let mut y = 0;
    x(async || {
        y += 1;
    })
    .await;
}
