rust
async fn bar() {
    let mut sum = 0;
    sum += block_on(async {
        baz().await;
    });
}

async fn baz() {
}
