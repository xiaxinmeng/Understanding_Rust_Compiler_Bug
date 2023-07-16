 rust
    #[simd_test(enable = "neon")]
    unsafe fn test_vbsl_s16() {
        let a = u16x4::new(u16::MAX, 0, 1, 2);
        let b = i16x4::new(i16::MAX, i16::MAX, i16::MAX, i16::MAX);
        let c = i16x4::new(i16::MIN, i16::MIN, i16::MIN, i16::MIN);
        let e = i16x4::new(i16::MAX, i16::MIN, i16::MIN | 1, i16::MIN | 2);
        let r: i16x4 = transmute(vbsl_s16(transmute(a), transmute(b), transmute(c)));
        assert_eq!(r, e);
    }
