rust
fn bar<'a>(foo: &'a mut Foo<'a>) {
    foo.modify();
}
