 rust
impl<'a, 'b> PartialEq<S<'b>> for S<'a> {
    fn eq(&self, b: &S<'b>) -> bool {
        true
    }
}
