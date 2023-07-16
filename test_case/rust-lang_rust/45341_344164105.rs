rust
struct Empty;

impl Display for Empty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("(empty)")
    }
}

impl<T: Display> Display for Foo<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}
