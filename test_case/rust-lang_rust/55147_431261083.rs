rust
pub fn checked(dst: &mut [u8], offset: usize) {
    if offset <= dst.len() {
        let dst = &mut dst[offset..];
        if dst.len() >= 4 {
            dst[0] = 1;
            dst[1] = 2;
            dst[2] = 3;
            dst[3] = 4;
        }
    }
}
