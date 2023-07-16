rust
// core::array

fn try_from_fn<T, const N: usize, R, F>(f: F) -> R::TryType<[T; N]>
where
    R: Residual,
    F: FnMut(usize) -> R::TryType<T>,
{ .. }
