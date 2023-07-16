rust
pub fn try_with<F, R>(&'static self, f: F) -> R
where
    F: FnOnce(Result<&T, AccessError>) -> R;
