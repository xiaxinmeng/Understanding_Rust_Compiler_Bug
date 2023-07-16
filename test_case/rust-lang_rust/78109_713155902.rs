rust
fn main() {
    let mut range = 0..=0;
    assert!(!range.is_empty());
    assert!(range.contains(&0));

    range.next();
    assert!(range.is_empty());
    assert!(!range.contains(&0));
}
