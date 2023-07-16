
trait Foo {
    type B: A = C;
    fn foo(&self) -> Option<Self::B>;
}

struct Bar;

impl Foo for Bar {
    fn foo(&self) -> Option<Self::B> {
        B { ... } // ~ERROR: expected associated type, found struct `B`
    }
}
