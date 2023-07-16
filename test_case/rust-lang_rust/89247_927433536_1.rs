rust
pub fn call_at_rt<R, F: FnOnce() -> R>(f: F) -> R;
