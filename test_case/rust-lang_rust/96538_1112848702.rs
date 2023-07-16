rust
        self.cast::<u8>().wrapping_sub(self_addr).wrapping_add(addr).cast::<T>()
