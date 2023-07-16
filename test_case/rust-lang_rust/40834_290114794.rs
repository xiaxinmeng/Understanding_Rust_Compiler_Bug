rust
fn foo_to_bar<'a>(foo: &'a Foo) -> &'a (Bar + 'a) {foo}
