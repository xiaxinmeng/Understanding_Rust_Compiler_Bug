
async fn foo() { ... }
...
tokio::spawn(async { foo() });
