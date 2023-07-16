rust
// Foo and Trait as before

struct Bar<T: ?Sized> {
    ptr: Box<Foo<T>>,
}

impl<T: ?Sized> PartialEq for Bar<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

fn eq(a: Bar<dyn Trait>, b: Bar<dyn Trait>) -> bool {
    a == b
}
