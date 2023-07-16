rust
fn block_on<F>(_: F) -> usize {
    0
}

fn main() {
    let mut sum = 0;
    sum += block_on(async {
        bar().await;
    });
}

async fn bar() { }
