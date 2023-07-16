rs
bitflags! {
    /// Used to tell `specific_from_str_radix` what kind of numbers it will get.
    struct Number: u32 {
        /// A number such as `+2` and `+64`.
        const PrefixedPositive = 0b00000001;
        /// A number such as `-2` and `-64`.
        const Negative = 0b00000010;
        /// A number such as `2` and `64`.
        const NonPrefixedPositive = 0b00000100;
        /// All kinds of numbers, such as `+2, `-64` and `2`.
        const All = Self::PrefixedPositive.bits | Self::Negative.bits | Self::NonPrefixedPositive.bits;
    }
}
