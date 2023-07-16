rust
pub fn then<T, F>(self, f: F) -> Option<T> where
    F: FnOnce() -> T, 
