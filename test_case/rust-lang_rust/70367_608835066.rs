rust
async fn bar() {
    let mut sum = 0;
    sum += {
        block_on(async {
            baz().await;
            let mut inner = 1;
            inner += block_on(async {
                baz().await;
                0
            })
        })
    };
}
