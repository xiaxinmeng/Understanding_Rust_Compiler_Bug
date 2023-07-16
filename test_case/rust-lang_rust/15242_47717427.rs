 rust
trait Foo {
    fn bar(self);
}

impl Foo for Box<Foo> { fn bar(self) { /* ... */ } }
