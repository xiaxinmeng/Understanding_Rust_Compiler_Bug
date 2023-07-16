rust
async fn noop() {}
async fn foo<T>(x: T) {
    std::mem::drop(x);
    noop().await;
}
fn assert_send(_: impl Send) {}
fn test<T>(x: T) {
    assert_send(foo(x));
}
