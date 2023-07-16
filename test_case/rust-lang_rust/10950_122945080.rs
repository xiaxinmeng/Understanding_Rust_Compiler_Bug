 rust
trait Foo {
    fn foo(&self, x: &Self);
}

trait Bar {
    type A: Foo;
    fn bar(&self) -> Self::A;
}

struct Wrap<T>(T);

impl<B: Bar> Wrap<B> {
    fn test(&self, x: &B::A) {
        self.0.bar().foo(x);
    }
}

fn main() {}
