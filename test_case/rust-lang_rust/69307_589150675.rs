rust
use tokio::runtime::Runtime;

fn main() {
    let mut rt = Runtime::new().unwrap();
    let mut sum = 0;
    sum = sum + rt.block_on(async {
        (async { }).await;
        1
    });
}
