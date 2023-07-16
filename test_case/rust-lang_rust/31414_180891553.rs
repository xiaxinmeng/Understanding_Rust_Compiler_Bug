 rust
impl Clone for Foo {
    fn clone(&self) -> Foo {
        if false { /* naive implementation */ }
        else { *self }
    }
}
