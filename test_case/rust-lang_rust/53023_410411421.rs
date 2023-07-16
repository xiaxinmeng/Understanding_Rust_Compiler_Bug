rust
pub fn my_u128_checked_mul_3(left: u128, right: u128) -> Option<u128> {
    unsafe {
        let [left_lo, left_hi] = std::mem::transmute::<u128, [u64; 2]>(left);
        let [right_lo, right_hi] = std::mem::transmute::<u128, [u64; 2]>(right);

        if left_hi.checked_mul(right_hi)? != 0 {
            return None
        }

        let a: u128 = (left_hi.checked_mul(right_lo)? as u128).shl(64);
        let b: u128 = (left_lo.checked_mul(right_hi)? as u128).shl(64);
        let c: u128 = (left_lo as u128) * (right_lo as u128);
        a.checked_add(b)?.checked_add(c)
    }
}
