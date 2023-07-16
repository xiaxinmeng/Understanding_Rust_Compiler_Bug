 rust
        impl<T: Copy + Default> Default for [T; $N] {
            #[inline]
            fn default() -> [T; $N] {
               [Default::default(); $N]
            }                                                                               
        }
