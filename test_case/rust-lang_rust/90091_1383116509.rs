rust
    pub fn pop<const N: usize, T: From<[u8; N]>>(&mut self) -> T {
        T::from(*self.bytes.split_array_ref().0)
    }
