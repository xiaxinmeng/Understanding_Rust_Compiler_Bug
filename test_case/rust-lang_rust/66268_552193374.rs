rust
pub fn replace2_mut(input: &mut [u8], from: u8, to: u8) {
    for b in input.iter_mut() {
        *b = if *b == from { to } else { *b };
    }
}
