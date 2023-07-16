rust
pub fn get4(array: &[u8; 8], x: usize, y: usize) -> u8 {
    if unlikely(x > 7) {
        0
    } else if unlikely(y > 7) {
        0
    } else {
        array[y]
    }
}
