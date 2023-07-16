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
        let (low_res, low_carry) = self.low.carrying_add(rhs.low, false); // Returns (0, true)

        // In the example from the main function, the operation below performes -128 + -1 + 1.
        // Intermediate overflow occurs as -128 + -1 gives -129, which overflow to 127. Then 127 + 1 gives 128, which overflows back to -128.
        // If carrying_add on signed numbers returns true on intermediate overflow, then this function returns true in the example.
        // However, from a i16 point of view, -32_513 + -255 gives -32768, which does not overflow. So intermediate overflow must not be returned.
        
        // Using overflowing_add as signed bit ints are not implemented anymore.
        let (high_res, high_carry) = self.high.overflowing_add(rhs.high + low_carry as i8); // Returns (-128, false), which is correct.

        self.low = low_res;
        self.high = high_res;
        high_carry
    }
}

fn main() {
    let (desired_res, desired_overflow) = (-32_513i16).overflowing_add(-255); // Returns (i16::MIN, false)
    let mut my_i16 = I16 { // -32_513
        low: 0xFF,
        high: -128,
    };
    let one = I16 { // -255
        low: 1,
        high: -1,
    };
    let my_i16_overflow = my_i16.overflowing_add(one); // If intermediate overflow was returned, the assert below would fail.

    let res = (my_i16.high as i16) << 8 | my_i16.low as i16;
    assert_eq!(desired_res, res); // Works
    assert_eq!(desired_overflow, my_i16_overflow);
}
