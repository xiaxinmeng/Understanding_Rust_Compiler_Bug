rust
pub fn then<T, R>(self, f: R) -> Option<T> where
    R: ResolveTo<T>,
