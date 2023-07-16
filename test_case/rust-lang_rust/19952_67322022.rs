 rust
#[cfg(not(stage0))]  // NOTE(stage0): Remove cfg after a snapshot
impl<'a> Add<String, String> for &'a str {
    fn add(self, mut other: String) -> String {
        other.push_str(self);
        other
    }
}
