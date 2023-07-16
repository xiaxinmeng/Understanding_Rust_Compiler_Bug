 rust
fn main() {
    let mut v = Vec::new();
    v.push(());
    v.push(());
    assert_eq!(v.len(), 2);
}
