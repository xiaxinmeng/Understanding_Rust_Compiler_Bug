 rust
trait Foo {
    type T;
}

trait Bar {
    fn bar(&self);
}

trait QuuxMyT {
    type MyT: Bar;
}

trait Quux: QuuxMyT + Foo<T=<Self as QuuxMyT>::MyT> {
    fn quux(&self) -> Self::T;
}

struct Baz<X> {
    x: X,
}

impl<X: Quux> Baz<X> {
    fn baz(&self) {
        self.x.quux().bar()
    }
}
