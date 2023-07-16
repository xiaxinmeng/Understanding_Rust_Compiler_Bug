rust
trait Bar {}
impl Bar for bool {}

fn baz() -> impl Bar {
    true;
}

fn main() {
    baz();
}
