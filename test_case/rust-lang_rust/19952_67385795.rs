 rust
#[cfg(not(stage0))]  // NOTE(stage0): Remove cfg after a snapshot
impl<'a> Add<&'a str, String> for String {
    fn add(mut self, other: &str) -> String {
        self.push_str(other);
        self
    }
}
