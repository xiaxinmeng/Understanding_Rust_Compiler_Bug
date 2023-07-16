rust
async fn test(a: &str) {}

fn test2(a: &str) {
    tokio::spawn(async move {
        test(a).await
    });
}

fn main() {
    let owned = "Hello!".to_string();
    test2(&owned);
}
