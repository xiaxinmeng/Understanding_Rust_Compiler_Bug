rust
async fn foo() {
  (Box::pin(foo()) as BoxFuture<()>).await
}
