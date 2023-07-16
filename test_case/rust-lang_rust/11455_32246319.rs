 rust
impl Iterable<int, ???> for int { // Always returns the same number.
    fn iter(&self) -> Iterator<int> {
        // Pretend this is an anonymous type that can't be constructed
        // outside of the iter method.
        struct Foo {x: int};
        impl Iterator<int> for Foo {
            fn next(&self) -> Option<int> {
                Some(self.x)
            }
        }
        Foo {x: *self}
    }
}
