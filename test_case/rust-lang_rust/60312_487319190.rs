rust
fn then<F, B>(self, f: F) -> Then<Self, B, F>
where
    F: FnOnce(Result<Self::Item, Self::Error>) -> B,
    B: IntoFuture,
    Self: Sized,
