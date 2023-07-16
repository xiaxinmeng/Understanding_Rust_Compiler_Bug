rust
fn try_map<F, U, R>(orig: Ref<'b, T>, f: F) -> Result<Ref<'b, U>, R::Error>
where
    F: FnOnce(&'b T) -> R,
    R: Try<Ok = &'b U>
