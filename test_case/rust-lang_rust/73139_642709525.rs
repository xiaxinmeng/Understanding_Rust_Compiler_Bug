rust
#[unstable(feature = "cstring_from_vec_with_nul", issue = "73179")]
impl TryFrom<Vec<u8>> for CString {
    type Error = FromBytesWithNulError;

    /// See the document about [`from_vec_with_nul`] for more
    /// informations about the behaviour of this method.
    ///
    /// [`from_vec_with_nul`]: struct.CString.html#method.from_vec_with_nul
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::from_vec_with_nul(value)
    }
}
