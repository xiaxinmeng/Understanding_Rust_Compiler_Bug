rust
trait Foo {
    /// default doc
    fn default_fn();
    fn another_one();
}

struct Bar;

impl Foo for Bar {
    fn default_fn() {}
    /// item doc!
    fn another_one() {}
}
