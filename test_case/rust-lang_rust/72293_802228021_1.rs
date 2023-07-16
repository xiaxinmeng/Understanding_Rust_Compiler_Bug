rust
struct Foo<'a>(&'a ());
impl<'a> Foo<'a> {
    const N: &'a () = &();
}
