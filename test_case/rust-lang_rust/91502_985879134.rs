rust
fn with_a(a: u64) -> Foo {
    let bar = BAR.get().unwrap();
    Foo {
        a,
        ..bar
    }
}
