rust
pub const fn is_ascii_punctuation_hybrid(byte: &u8) -> bool {
    const LOOKUP : [bool; 128] = ...;

    byte.is_ascii() && LOOKUP[*byte as usize]
}
