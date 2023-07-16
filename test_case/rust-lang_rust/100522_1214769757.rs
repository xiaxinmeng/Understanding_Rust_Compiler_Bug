rust
        #[inline(always)]
        fn inner<const N: usize, T: Tr>() {
            inner::<N, T::Next>();
            inner::<N, T::Next>();
        }
