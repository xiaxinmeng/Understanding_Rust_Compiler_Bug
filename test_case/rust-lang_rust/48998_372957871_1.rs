rust
fn foo_me<'a>() -> Foo<'a> {
    Foo {
        bar: &Bar::new(42)
    }
}
