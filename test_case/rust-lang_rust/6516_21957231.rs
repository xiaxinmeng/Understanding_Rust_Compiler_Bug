 rust
struct Foo<T> {
    value: T
}

impl<T: Send> Foo<T> {
    fn from_owned(value: T) -> Foo<T> {
        Foo{value: value}
    }
}

impl<T: Freeze> Foo<T> {
    fn from_const(value: T) -> Foo<T> {
        Foo{value: value}
    }
}

fn main() {
}
