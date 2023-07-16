rust
fn try_find<F, E, T, FR, R>(&mut self, mut f: F) -> R where
        Self: Sized,
        FR: Try<Ok = bool, Error = E>,
        F: FnMut(&Self::Item) -> FR,
        R: Try<Ok = Option<Self::Item>, Error = E> { ... }
