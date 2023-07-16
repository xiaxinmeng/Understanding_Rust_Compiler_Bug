 rust
impl Index<usize> for str {
    type Output = char;
    fn index(&self, index: usize) -> &char {
        let c = self[index..].chars().next().unwrap()
        return &c; // c doesn't live long enough.
    }
}
