rust
impl OsStr {
    fn from_bytes_checked(_: &[u8]) -> Result<&OsStr, Utf8Error>;
}
