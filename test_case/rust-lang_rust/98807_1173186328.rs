rust
struct Foo;

trait Bar {}

trait Qux {}

impl<T> Bar for Option<T> where T: Qux {}

fn needs_bar(t: impl Bar) {}

fn main() {
    needs_bar(Some(Foo));
}
