rust
fn checked_sum<T: CheckedAdd + Zero>(a: impl AsRef<[T]>) -> Option<T> {
    a.as_ref().iter().try_fold(T::zero(), |a, b| a.checked_add(b))
}
