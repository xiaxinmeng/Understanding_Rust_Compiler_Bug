rust
// in libcore
impl {i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize} {
    pub fn div_euclid(self, rhs: Self) -> Self;
    pub fn rem_euclid(self, rhs: Self) -> Self;
    pub fn wrapping_div_euclid(self, rhs: Self) -> Self;
    pub fn wrapping_rem_euclid(self, rhs: Self) -> Self;
    pub fn checked_div_euclid(self, rhs: Self) -> Option<Self>;
    pub fn checked_rem_euclid(self, rhs: Self) -> Option<Self>;
    pub fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool);
    pub fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool);
}

// in libstd
impl {f32, f64} {
    pub fn div_euclid(self, rhs: Self) -> Self;
    pub fn rem_euclid(self, rhs: Self) -> Self;
}
