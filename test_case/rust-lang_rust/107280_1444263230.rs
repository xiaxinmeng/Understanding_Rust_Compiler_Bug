rs
async fn foo() {
    inner::<false>().await
}

async fn inner<T, const PING: bool>() {}
