rust
fn foo(drop_me: Box<Foo>) {
    drop(drop_me);
}
