rust
fn foo() -> impl Bar {
    if bar() {
        Baz::new()
    } else {
        panic!(“diverge”)
    }
}
