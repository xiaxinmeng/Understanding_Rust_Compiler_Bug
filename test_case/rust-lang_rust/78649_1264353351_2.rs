rust
async fn foo() {
  #[inline(always)]
  fn foo_recurse() -> futures::future::BoxFuture<()> {
    Box::pin(foo())
  }
  foo_recurse().await
}
