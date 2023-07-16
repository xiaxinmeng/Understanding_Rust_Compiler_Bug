
        pub fn next_power_of_two(self) -> Self {
            let bits = size_of::<Self>() * 8;
            let one: Self = Self::one();
            one << ((bits - self.wrapping_sub(one).leading_zeros() as usize) % bits)
        }
