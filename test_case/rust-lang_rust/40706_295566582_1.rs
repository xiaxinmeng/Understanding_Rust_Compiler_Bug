
    fn next_power_of_two(self) -> Self {
        self
            .wrapping_sub(1)
            .round_up_to_one_less_than_a_power_of_two()
            .wrapping_add(if self == 0 { 1 } else { 0 })
            + 1
    }
