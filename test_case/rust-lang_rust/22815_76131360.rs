 rust
trait Foo {
    type A;
    fn foo(&self) {}
}

impl Foo for usize {
    type A = usize;
}

struct Bar<T: Foo> { inner: T::A }

fn is_send<T: Send>() {}

fn main() {
    is_send::<Bar<usize>>();
}
