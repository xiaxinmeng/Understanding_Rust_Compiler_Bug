 rust
trait ExactSizeHint: Iterator {
    #[test]
    fn test_exact_size_hint() {
        let it = make_a_self(); // the hard part
        assert!(match it.size_hint() {
            (n, Some(m)) => n == m,
            _ => false
        })
    }
}
