rust
#[derive(Clone, PartialEq, Eq, Debug)]
#[unstable(feature = "cstring_from_vec_with_nul", issue = "73179")]
pub struct FromVecWithNulError {
    error_kind: FromBytesWithNulErrorKind,
    bytes: Vec<u8>,
}
