rust
// This is fast
pub fn stdlib_max<T: Ord + Copy>(a: &[T]) -> Option<T> {
    a.iter().copied().max()
}

pub fn demo(z: &[i32]) -> Option<i32> {
    stdlib_max(z)
}
