rust
impl PartialEq for X {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl Eq for X {}
