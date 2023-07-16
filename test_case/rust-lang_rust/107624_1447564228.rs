rust
impl CStr {
    pub const fn from_bytes_with_nul(bytes: &[u8]) -> Result<&Self, FromBytesWithNulError>
    pub const fn to_bytes(&self) -> &[u8]
    pub const fn to_bytes_with_nul(&self) -> &[u8]
    pub const fn to_str(&self) -> Result<&str, str::Utf8Error>
}
