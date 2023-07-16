rust
pub fn copy_c(dest: &mut [u8], src: &[u8]) {
    let len = std::cmp::min(dest.len(), src.len());
    let (left, _) = dest.split_at_mut(len);
    left.copy_from_slice(&src[..len]);
}
