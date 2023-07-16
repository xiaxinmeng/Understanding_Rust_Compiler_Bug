rust
pub const fn is_ascii_punctuation_lookup_bitset(byte: &u8) -> bool {
    const LOOKUP : [u8; 32] = ...;

    LOOKUP[(*byte / 8) as usize] >> (*byte % 8) & 1u8 == 1
}
