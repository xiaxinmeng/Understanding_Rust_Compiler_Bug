rust
impl [T; N]
    fn split<N2, N3>(&self) -> (&[T; N2] &[T; N3]) {
        assert_eq!(N, N2 + N3);
        // ...
    }
}
