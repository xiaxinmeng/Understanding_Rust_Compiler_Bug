rust
trait Foo {
    type Inner = ();
    type Outer: Into<Self::Inner>;
}

impl Foo for () {
    // With this, it compiles:
    // type Inner = ();
    type Outer = ();
}
