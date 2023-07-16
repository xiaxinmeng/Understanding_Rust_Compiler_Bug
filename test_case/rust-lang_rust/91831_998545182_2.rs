rust
fn bar<'a, 'b>(foo: &'a mut Foo<'b>) {
    foo.modify();
}
