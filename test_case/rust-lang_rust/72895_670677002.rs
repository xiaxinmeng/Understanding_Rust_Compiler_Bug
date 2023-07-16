rust
pub fn is_ascii_punctuation_hybrid_bitset(byte: &u8) -> bool {
    const LOOKUP: u128 = 79753679825085174867510150554292584448;
    
    LOOKUP >> *byte & 1 == 1
}
