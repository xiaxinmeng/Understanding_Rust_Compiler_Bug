 rust
trait Foo {
    fn foo_fn(self);
}

struct NoData;

impl NoData where NoData: Foo {
    fn any_fn(self) {
        self.foo_fn()
    }
}

fn main() { }
