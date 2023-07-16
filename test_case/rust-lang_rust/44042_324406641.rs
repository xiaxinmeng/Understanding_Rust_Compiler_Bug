rust
// The existing impl block
#[lang = "slice"]
impl<T> [T] {
    // ...

    // New methods:
    fn is_ascii(&self) where Self: AsRef<[u8]> {
        let self_as_bytes = self.as_ref();
        // ...
    }
}
