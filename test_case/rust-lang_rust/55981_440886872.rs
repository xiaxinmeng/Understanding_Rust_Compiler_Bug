rust
// when inner ptr is dangling, returns None.
pub fn weak_count(this: &Weak<T>) -> Option<usize>
