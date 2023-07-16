rust
    /// Check that the two arguments are the same size.  Otherwise, fails at compile-time.
    macro_rules! static_assert_same_size {
        ($a:ty, $b:ty) => {
            (if false {
                let _: $a = unsafe { std::mem::transmute(std::mem::zeroed::<$b>()) };
            })
        }
    }
    