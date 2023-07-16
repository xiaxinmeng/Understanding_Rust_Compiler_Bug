rust
pub fn foo(s: &[&[u32]]) {
    let n = s.len();

    assert!(n <= 40);

    for i in 0..n {
        assert!(i < 40); // Not optimized.

        if i % 2 == 0 {
            s[i][0];
        }
    }
}
