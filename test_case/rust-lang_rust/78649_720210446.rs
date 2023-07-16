rust
async fn foo() {
    Box::new(async {
        foo().await;
    });
}
