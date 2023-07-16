rust
pub const fn is_ascii_punctuation_branch(byte: &u8) -> bool {
    matches!(*byte, b'!'..=b'/' | b':'..=b'@' | b'['..=b'`' | b'{'..=b'~')
}
