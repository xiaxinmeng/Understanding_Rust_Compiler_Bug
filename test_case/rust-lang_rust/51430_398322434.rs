rust
impl From<Foo> for Bar {
    /// Converts a `Foo` into a `Bar`.
    /// This conversion does not allocate.
    fn from(f: Foo) -> Bar {
        //....
    }
}
