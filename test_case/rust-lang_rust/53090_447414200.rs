rust
trait Foo {
    type Assoc;
}

struct Bar;
impl Foo for Bar {
    existential type Assoc;
}
