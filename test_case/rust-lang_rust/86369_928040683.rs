rust
impl<T: AsRef<[u8]>> Cursor<&'a [u8]> {
    pub fn remaining_slice<'b>(&'b self) -> &'b [u8] {

        let slice: &'b &'a [u8] = AsRef::as_ref(&'b self.inner);

        let slice: &'a [u8] = *slice;
        // where 'a: 'b

        slice
    }
}
