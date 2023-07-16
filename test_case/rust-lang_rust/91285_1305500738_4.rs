rs
fn try_from_fn<T, const N: usize, R, F>(f: F)
// -> <R::Residual as Residual>::TryTypeWithOutput<[T; N]>
   -> ChangeOutputType<R, [T; N]>
where
    F : FnMut(usize) -> R,
    R : Try<Output = T>,
    R::Residual : Residual, // EDIT
