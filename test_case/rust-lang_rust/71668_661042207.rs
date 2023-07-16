rust
#![forbid(unsafe_code)]

trait Collection<E> {
    /// # Safety
    /// `index` must be a valid index into this collection
    unsafe fn get_unchecked(&self, index: usize) -> E;
}
struct Bucket<T> {
    value: T,
}
impl<T: Copy> Collection<T> for Bucket<T> {
    unsafe fn get_unchecked(&self, _: usize) -> T {
        self.value
    }
}
