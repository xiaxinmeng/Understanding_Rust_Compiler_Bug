rust
fn update<F>(#[exclusive_borrow] &self, f: F) -> T where F: FnOnce(T) -> T
