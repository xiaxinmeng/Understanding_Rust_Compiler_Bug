rust
                #[inline]
                fn forward(&self, n: usize) -> Option<Self> {
                    match Self::try_from(n) {
                        Ok(n_converted) => self.checked_add(n_converted),
                        Err(_) => None,  // if n is out of range, `something_unsigned + n` is too
                    }
                }
