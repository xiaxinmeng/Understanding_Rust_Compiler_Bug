 rust
trait Foo: Clone {}

impl Foo for fn(&int) {}

fn foo(_: &int) {}

fn main() {
    foo.clone();
}
