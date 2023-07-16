 rust
trait Foo {
    fn bar(&self); // A
}

impl Foo for uint {
    fn bar(&self) { // B
        self.bar(); // C
    }
}
