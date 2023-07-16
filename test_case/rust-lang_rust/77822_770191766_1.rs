rust
pub fn foo(y: &[u32]) {
    let mut x = 0;
    for (c, _y) in y.iter().enumerate() {
        assert!(c < y.len());
        x = c;
    }
    assert!(x == y.len());
}
