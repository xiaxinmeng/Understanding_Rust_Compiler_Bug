 rust
fn main() {
    let a = 0.57 * 100f64;
    let b = 57f64;
    assert_eq!(format!("{:?}", a), format!("{:?}", b));
    assert!(a != b)
}
