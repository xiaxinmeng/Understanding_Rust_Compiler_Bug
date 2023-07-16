rust
trait Bar {}

trait Foo {
    type Assoc: Bar;
}

impl Foo for () {
    type Assoc = bool;
}
