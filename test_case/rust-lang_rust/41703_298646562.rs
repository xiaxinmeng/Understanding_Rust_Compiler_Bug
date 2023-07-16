rust
pub trait OsStringExt {
    fn from_wide(wide: &[u16]) -> Self;
}

pub trait OsStrExt {
    fn encode_wide(&self) -> EncodeWide;
}
