rust
fn and_option_from<T, F: FnOnce() -> Option<T>>(self, f: F) -> Option<T> 
