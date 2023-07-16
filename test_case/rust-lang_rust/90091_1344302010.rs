rust
    pub fn pop<const N: usize, T: From<[u8; N]>>(&mut self) -> T {
        // TODO switch to `rsplit_array_mut`, once https://github.com/rust-lang/rust/issues/90091 stabilizes
        let bytes = self.bytes.split_off(self.bytes.len() - N);
        T::from(<[u8; N]>::try_from(bytes).unwrap())
    }
