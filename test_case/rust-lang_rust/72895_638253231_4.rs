rust
pub fn is_ascii_punctuation_linear_search(byte: &u8) -> bool {
    const LOOKUP : [u8; 32] = ...;

    LOOKUP.contains(byte)
}
