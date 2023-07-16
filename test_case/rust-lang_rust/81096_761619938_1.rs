rust
async fn test(a: &str) {}

fn test2(a: &str) {
    let a = a.to_owned();
    tokio::spawn(async move {
        test(&a).await
    });
}

#[tokio::main]
async fn main() {
    let owned = "Hello!".to_string();
    test2(&owned);
}
