rust
trait Field {
    fn field(&self) {}
}

impl<T> Field for T {}

#[derive(Debug)]
struct Foo(());

fn main() {
    Foo(()).field();
}
