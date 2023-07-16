rust
        pub fn next_power_of_two(self) -> Self {
            let bits = size_of::<Self>() * 8;
            let one: Self = 1;
            one << ((bits - self.wrapping_sub(one).leading_zeros() as usize) % bits)
        }
