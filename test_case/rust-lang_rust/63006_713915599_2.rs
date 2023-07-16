rust
pub fn clippy_suggestion() -> [u32; 32] {
    let mut odd = [0u32; 32];
    
    odd[0] = 0xedb88320;
    for (i, odd_val) in odd.iter_mut().skip(1).enumerate() {
        *odd_val = 1 << i;
    }
    odd
}
