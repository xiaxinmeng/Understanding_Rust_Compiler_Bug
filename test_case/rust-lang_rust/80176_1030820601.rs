rust
trait Foo {
    fn f<'a>(_: &'a ());
}

impl Foo for () {
    fn f(_: &'static ()) {}
}
