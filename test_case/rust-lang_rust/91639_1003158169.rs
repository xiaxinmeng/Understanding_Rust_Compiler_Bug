rust
struct Foo<'a>(&'a i32);

impl<'a> Foo<'a> {
    fn modify(&'a mut self) {}
}

fn bar(foo: &mut Foo) {
    foo.modify();
}
