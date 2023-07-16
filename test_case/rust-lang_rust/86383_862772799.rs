
pub fn bound(index: usize, slice: &[u8]) -> u8 {
    if index < slice.len() {
        slice[index]
    } else {
        42
    }
}
