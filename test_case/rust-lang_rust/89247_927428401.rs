rust
pub fn call_if_rt<R, F: FnOnce() -> R + Copy>(f: F) -> Option<R>;
