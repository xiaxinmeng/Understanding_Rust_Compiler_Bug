rust
fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
where
    Self: Sized,
    F: FnMut(B, Self::Item) -> R,
    R: Try<Ok = B>
