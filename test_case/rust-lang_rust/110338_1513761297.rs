rust
fn foo() -> impl Future<Output = i64> { // effectively the same signature as `async fn foo() -> i64 { ... }`
    async {
        ready(10).await
    }
}
