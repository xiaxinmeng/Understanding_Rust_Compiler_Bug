rust
impl bool {
    pub fn then_some<T>(self, t: T) -> Option<T> {…}
    pub fn then<T, F: FnOnce() -> T>(self, f: F) -> Option<T> {…}
}
