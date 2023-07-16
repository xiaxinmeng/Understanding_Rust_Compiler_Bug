rs
fn try_from_fn<R, const N: usize, F>(cb: F)
  -> ChangeOutputType<R, [R::Output; N]>
where
    F : FnMut(usize) -> R,
    R : Try,
    R::Residual : Residual<[R::Output; N]>,
