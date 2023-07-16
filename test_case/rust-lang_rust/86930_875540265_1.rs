rust
macro_rules! bench {
    ($T:ident, $old:ident, $new:ident) => {
        // for example for u32, compute:
        //   * 1, 2, 3, ..., 255
        //   * 1 << 8, 2 << 8, 3 << 8, ..., 255 << 8
        //   * 1 << 16, 2 << 16, 3 << 16, ..., 255 << 16
        //   * 1 << 24, 2 << 24, 3 << 24, ..., 255 << 24

        #[bench]
        fn $old(bench: &mut Bencher) {
            for n in 0..(<$T>::BITS / 8) {
                bench.iter(|| {
                    for i in 1..=(u8::MAX as $T) {
                        let i = i << (n * 8);
                        black_box(old::$T(i, 10));
                    }
                });
            }
        }

        #[bench]
        fn $new(bench: &mut Bencher) {
            for n in 0..(<$T>::BITS / 8) {
                bench.iter(|| {
                    for i in 1..=(u8::MAX as $T) {
                        let i = i << (n * 8);
                        black_box(int_log10::$T(i));
                    }
                });
            }
        }
    };
}

bench! { u8, u8_old, u8_new }
bench! { u16, u16_old, u16_new }
bench! { u32, u32_old, u32_new }
bench! { u64, u64_old, u64_new }
bench! { u128, u128_old, u128_new }
