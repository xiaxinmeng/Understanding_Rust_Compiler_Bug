rust
            #[stable(feature = "int_to_from_bytes", since = "1.32.0")]
            #[rustc_const_stable(feature = "const_int_conversion", since = "1.43.0")]
            #[inline]
            pub const fn from_ne_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
                #[allow_internal_unstable(const_transmute)]
                macro_rules! m {
                    () => {
                        // SAFETY: ...
                        unsafe { mem::transmute(bytes) }
                    }
                }
                m!()
            }
