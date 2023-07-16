
trait Foo {
    type Bar<'a>;
    fn baz<'a>(&self, val: Self::Bar<'a>);
}
impl Foo for Qux {
    type Bar<'a> = &'a [u8];
    fn baz<'a>(&self, val: &'a [u8]) {}
}
