rust
pub const fn is_ascii_punctuation_branch_adapted(byte: &u8) -> bool {
    // !byte.is_alphanumeric() && byte.is_graphic()
    !matches!(*byte, b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z') && matches!(*byte, b'!'..=b'~')
}
