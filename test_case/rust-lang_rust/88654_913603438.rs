rust
#![no_std]

pub struct Decimal {
    hi: u32,
    lo: u32,
}

impl Decimal {
    pub fn checked_mul(self) -> Decimal {
        let mut tmp = self.lo as u64 * self.lo as u64;
        let mut tmp2 = (self.lo as u64 * self.lo as u64).wrapping_add(tmp >> 32);
        tmp = tmp.wrapping_add(tmp2);

        if tmp < tmp2 {
            tmp2 = (tmp >> 32) | (1u64 << 32);
        } else {
            tmp2 = tmp >> 32;
        }

        tmp = (self.lo as u64 * self.lo as u64) + tmp2;
        if self.lo > 0 {
            tmp2 = self.hi as u64 * self.lo as u64;
            tmp = tmp.wrapping_add(tmp2);
        }

        Decimal {
            lo: tmp as u32,
            hi: tmp2 as u32,
        }
    }
}
