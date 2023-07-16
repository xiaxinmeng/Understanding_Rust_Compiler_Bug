rust
pub fn is_ascii_punctuation_binary_search(byte: &u8) -> bool {
    const LOOKUP : [u8; 32] = ...;

    LOOKUP.binary_search(byte).is_ok()
}
