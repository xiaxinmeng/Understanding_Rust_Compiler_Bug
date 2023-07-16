rust
fn main() {
    let a = true;
    let _ = &a;
    let mut b = false;
    b |= a;
    assert!(b);
}
