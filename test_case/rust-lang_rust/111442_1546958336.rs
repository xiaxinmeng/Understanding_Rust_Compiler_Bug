rust
        pub const fn checked_ilog2(self) -> Option<u32> {
            if let Some(x) = <$NonZeroT>::new(self) {
                Some(x.ilog2())
            } else {
                None
            }
        }
