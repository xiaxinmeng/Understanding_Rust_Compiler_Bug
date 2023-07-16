rust
    fn instant_math() {
        let a = Instant::now();
        let b = Instant::now();
        let dur = b.duration_since(a);
        assert_almost_eq!(b - dur, a);
        assert_almost_eq!(a + dur, b);

        let second = Duration::new(1, 0);
        assert_almost_eq!(a - second + second, a);
    }
