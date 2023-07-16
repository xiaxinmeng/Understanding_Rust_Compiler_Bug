rust
impl<'a> IoSlice<'a> {
    pub fn inner_ref(&self) -> &'a [u8] {
        // ...
    }
}
