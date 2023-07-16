rs
#![feature(bigint_helper_methods)]
#![feature(array_zip)]

#[derive(Clone, Copy)]
struct SignedBigInt<const N: usize> {
    pub lows: [u8; N],
    pub high: i8,
}

impl<const N: usize> SignedBigInt<N> {
    /// Subtracts `rhs` from `self` and returns true if signed overflow occurs, false otherwise.
    pub fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let mut borrow = false;

        let lows = self.lows.zip(rhs.lows).map(|(left, right)| {
            let (res, b) = left.borrowing_sub(right, borrow);
            borrow = b;
            res
        });

        let (high, b) = self.high.borrowing_sub(rhs.high, borrow);

        (Self { lows, high }, b)
    }
}

fn main() {
    let left = SignedBigInt { // -32_768
        lows: [0],
        high: -128,
    };
    let right = SignedBigInt { // 127
        lows: [127],
        high: 0,
    };
    let (res, borrow) = left.overflowing_sub(right); // -32_895 overflow to 32_641

    assert_eq!(res.high, 0x7F);
    assert_eq!(res.lows[0], 0x81);
    assert_eq!(borrow, true);
}
