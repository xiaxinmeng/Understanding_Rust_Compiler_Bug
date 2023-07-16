rust
async fn something<B: Borrow<Foo>>(b: B) {
   other(b.borrow()).await;
}