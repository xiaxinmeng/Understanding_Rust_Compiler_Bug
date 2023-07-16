rust
#[must_not_suspend]
struct No;

async fn shushspend() {}

async fn wheeee<T>(t: T) {
    shushspend().await;
    drop(t);
}

async fn yes() {
    wheeee(No).await;
}
