rust
pub fn copy_s(dest: &mut [u8], src: &[u8]) {
    let len = std::cmp::min(dest.len(), src.len());
    for i in 0..src.len() {
        unsafe {
            *dest.get_unchecked_mut(i) = *src.get_unchecked(i);
        }
    }
}
