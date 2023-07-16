rust
pub fn test(dwords: [u32; 2]) {
    match dwords {
        [0 | 1, _a] => (),
        _ => (),
    }
}
