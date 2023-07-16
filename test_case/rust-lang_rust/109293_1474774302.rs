rust
pub fn foo(n: usize, s: &[&[u32]]) {
    assert!(n <= 40);

    s.iter().map(|x| x[0]).sum::<u32>();

    for i in 0..n {
        assert!(i < 40); // Not optimized.
    }
}
