rust
trait Foo {
    fn bar();
}

impl Foo for () {
    fn bar(s: _) {}
}
