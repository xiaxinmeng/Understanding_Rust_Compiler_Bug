rust
#[unstable(feature = "cstring_from_vec_with_nul", issue = "73179")]
impl fmt::Display for FromVecWithNulError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.error_kind {
            FromBytesWithNulErrorKind::InteriorNul(pos) => {
                write!(f, "data provided contains an interior nul byte at pos {}", pos)
            }
            FromBytesWithNulErrorKind::NotNulTerminated => {
                write!(f, "data provided is not nul terminated")
            }
        }
    }
}
