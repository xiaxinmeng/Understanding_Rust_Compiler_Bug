rust
struct Foo<T: ?Sized> {
    value: T,
}

impl<T: ?Sized> PartialEq for Foo<T> {
    fn eq(&self, _: &Self) -> bool { true }
}

trait Trait {}

struct Bar {
    ptr: Box<Foo<Trait>>,
}

impl PartialEq for Bar {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

fn main() {}
