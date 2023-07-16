rust
use futures::executor::block_on;

fn main() {
    let mut sum = 0;
    sum += block_on(async {
        (async { }).await;
        1
    });
}
