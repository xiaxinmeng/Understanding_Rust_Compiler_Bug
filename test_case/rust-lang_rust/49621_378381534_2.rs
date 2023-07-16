rust
fn bar(mut foo: Pin<Foo>) {
     Pin::borrow(&mut foo).foo()
}
