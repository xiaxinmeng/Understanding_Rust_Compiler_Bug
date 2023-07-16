rs
fn try_from_fn<T, const N: usize, F, Ret, Res>(f: F)
  -> Res::TryTypeWithOutput<[T; N]>
where
    F : FnMut(usize) -> Ret,
    Ret : Try<Output = T, Residual = Res>,
    Res : Residual, // EDIT
