rust
fn foo() -> BoxFuture<()> {
  Box::pin(async move {
    foo().await
  }) as BoxFuture<()>
}
