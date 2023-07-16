rust
impl CString {
    #[unstable(feature = "cstring_from_vec_with_nul", issue = "73179")]
     pub fn from_vec_with_nul(v: Vec<u8>) -> Result<Self, FromBytesWithNulError> { /* ... */ }
}
