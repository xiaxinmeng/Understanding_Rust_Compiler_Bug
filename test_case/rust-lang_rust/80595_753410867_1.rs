rust
pub(crate) enum KeyType {
    Short(char),
    Long(OsString),
    Position(u64),
}

impl PartialEq<char> for KeyType {
    fn eq(&self, rhs: &char) -> bool {
        match self {
            KeyType::Short(c) => c == rhs,
            _ => false,
        }
    }
}
