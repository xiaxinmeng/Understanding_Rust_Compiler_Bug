rust
fn with_environment<F, R>(&mut self, f: F) -> R
where
    F: FnOnce(RangeMut<K, V>, RangeMut<K, V>) -> R
