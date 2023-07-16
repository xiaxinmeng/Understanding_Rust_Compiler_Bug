rust
pub fn generic_copy<T: Clone>(dest: &mut [T], src: &[T]) {
    let len = std::cmp::min(dest.len(), src.len());
    let (dest, _) = dest.split_at_mut(len);
    let src = &src[..len];
    
    for i in 0..src.len() {
        dest[i] = src[i].clone()
    }
}

pub fn concrete(dest: &mut [u8], src: &[u8]) {
    generic_copy(dest, src)
}
