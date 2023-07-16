rust
pub fn original() -> [u32; 32] {
    let mut odd = [0u32; 32];
    let mut row: u32;
    
    odd[0] = 0xedb88320;
    row = 1;
    for n in 1..32 {
        odd[n] = row;
        row <<= 1;
    }
    odd
}
