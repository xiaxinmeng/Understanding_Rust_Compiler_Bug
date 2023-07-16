rust
fn foo<F>(f: F)
    where F: Fn() -> Foo + Send
{ }
