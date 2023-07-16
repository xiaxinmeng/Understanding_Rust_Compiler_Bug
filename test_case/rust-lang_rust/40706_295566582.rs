
    fn round_up_to_one_less_than_a_power_of_two(self) -> Self {
        let bits = std::mem::size_of::<Self>() as u32 * 8;
        let z = self.leading_zeros();
        (if z == bits { 0 as Self } else { !0 }).wrapping_shr(z)
    }
