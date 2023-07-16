rust
trait Zoo {
    fn foo(self) where Self: Clone {}
}

struct S {}
impl Zoo for S {}

fn example(x: Box<dyn Zoo>) {
    x.foo();
}
