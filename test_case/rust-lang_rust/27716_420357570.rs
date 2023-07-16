rust
pub fn try_with<F, R>(&'static self, f: F) -> Result<R, AccessError>
where
    F: FnOnce(&T) -> R;
