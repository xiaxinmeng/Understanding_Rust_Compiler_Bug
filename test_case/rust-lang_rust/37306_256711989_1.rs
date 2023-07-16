 rust

pub fn map_fast(l: &[(u32, u32)]) -> Vec<u32> {
    let mut result = Vec::with_capacity(l.len());
    for i in 0..l.len() {
        unsafe {
            *result.get_unchecked_mut(i) = l[i].0;
            result.set_len(i);
        }
    }
    result
}
