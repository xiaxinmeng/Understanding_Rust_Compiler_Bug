rs
#![feature(bigint_helper_methods)]
struct I16 {
    /// Low-order bytes has to be unsigned.
    pub low: u8,
    /// Most Significant Byte has to be of the same signedness as the desired type.
    /// So u8 if I implement a U16, i8 if I implement I16.
    pub high: i8,
}

impl I16 {
    /// Adds `rhs` to `self` and returns true if signed overflow occurs, false otherwise.
    pub fn overflowing_add(&mut self, rhs: Self) -> bool {
        // For the low-order bytes, unsigned overflow has to be used.
        let (low_res, low_carry) = self.low.carrying_add(rhs.low, false);

        // To  correctly detect signed overflow, carrying_add should also detect signed-overflow with signed data,
        // Which is currently not the case. This returns (0x80, false) whereas a signed overflow occured.
        let (high_res, high_carry) = self.high.carrying_add(rhs.high, low_carry);

        self.low = low_res;
        self.high = high_res;
        high_carry
    }
}

fn main() {
    let (desired_res, desired_overflow) = 0x7FFFi16.overflowing_add(1); // returns (i16::MIN, true)
    let mut my_i16 = I16 {
        low: 0xFF,
        high: 0x7F,
    };
    let one = I16 {
        low: 1,
        high: 0,
    };
    let my_i16_overflow = my_i16.overflowing_add(one);

    let res = (my_i16.high as i16) << 8 | my_i16.low as i16;
    assert_eq!(desired_res, res); // Works
    assert_eq!(desired_overflow, my_i16_overflow); // Fails with left == true and right == false
}
