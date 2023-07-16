rust
async fn f() {
    let arena = 0;
    async move { &arena }.await;
}
