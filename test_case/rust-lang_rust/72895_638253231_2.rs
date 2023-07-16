rust
pub const fn is_ascii_punctuation_lookup(byte: &u8) -> bool {
    const LOOKUP : [bool; 256] = ...;

    LOOKUP[*byte as usize]
}
