rust
pub fn slicing() -> [u32; 32] {
    let mut odd = [0u32; 32];
    
    odd[0] = 0xedb88320;
    for (i, odd_val) in odd[1..].iter_mut().enumerate() {
        *odd_val = 1 << i;
    }
    odd
}
