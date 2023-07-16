rust
pub fn insert(target: &mut String, index: usize, inserted: &str) {
    for c in inserted.chars().rev() {
        target.insert(index, c);
    }
}
