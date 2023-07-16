rust
impl<F: Fn(&mut ())> Hello<F> {
//    ^^^^^^^^^^^^^
    pub fn new(f: F) -> Self {
        Self(f)
    }
}
