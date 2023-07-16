rust
enum Foo<T> {
    T(T),
    Empty,
}

impl<T: Display> Display for Foo<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Foo::T(ref t) => Display::fmt(t, f),
            Foo::Empty => f.write_str("(empty)"),
        }
    }
}
