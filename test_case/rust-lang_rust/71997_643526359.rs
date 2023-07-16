rust
pub fn copy_t(dest: &mut [u8], src: &[u8]) {
    let len = std::cmp::min(dest.len(), src.len());
    for i in 0..len {
        dest[i] = src[i]
    }
}
