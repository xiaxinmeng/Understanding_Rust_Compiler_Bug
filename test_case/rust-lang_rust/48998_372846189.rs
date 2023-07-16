rust
fn foo_me<'a>() -> Foo<'a> {
    static BAR: Bar = Bar {
        val: 42
    };
    Foo {
        bar: &BAR
    }
}
