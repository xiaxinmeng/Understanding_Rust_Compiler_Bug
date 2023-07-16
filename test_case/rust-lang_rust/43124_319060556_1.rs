rust
pub fn create_with_range() -> Vec<T> {
    let mut arr = vec![0;SIZE as usize];
    for n in 0..SIZE {
        unsafe {
            *arr.get_unchecked_mut(n as usize) = n;
        }
    }
    arr
}
