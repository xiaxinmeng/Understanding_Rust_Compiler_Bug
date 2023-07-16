rust
impl Join<&str> for [char] {
    type Output = String;
    fn join(slice: &Self, sep: &str) -> String { … }
}
