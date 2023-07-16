rust
pub fn func(inbounds: &Enum, array: &[i16; 3]) -> i16 {
    array[*inbounds as usize]
}
