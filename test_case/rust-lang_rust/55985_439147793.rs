rust
fn main() {
    let mut it = (0_usize..640).step_by(128);
    it.nth(0);
    assert_eq!(it.next(), Some(128));
}
