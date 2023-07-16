rust
pub const fn is_ascii_punctuation_hybrid_bitset(byte: &u8) -> bool {
    const LOOKUP : [u8; 16] = ...;

    byte.is_ascii() && LOOKUP[(*byte / 8) as usize] >> (*byte % 8) & 1u8 == 1
}
