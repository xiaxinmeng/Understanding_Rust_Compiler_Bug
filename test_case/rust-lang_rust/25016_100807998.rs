 rust
fn main() {
    let v = [(); 10];
    let mut it = v.iter();
    let _ = it.nth(4);
    assert_eq!(it.count(), 5);
}
