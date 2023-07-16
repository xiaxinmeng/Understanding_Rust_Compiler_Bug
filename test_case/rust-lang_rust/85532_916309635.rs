
/// Extended multiply-addition of `(lhs * rhs) + add`. The result is returned as a tuple of the wrapping part and the
/// overflow part. No numerical overflow is possible even if all three arguments are set to their max values.
pub const fn widen_mul_add(lhs: u128, rhs: u128, add: u128) -> (u128, u128) {
    //                       [rhs_hi]  [rhs_lo]
    //                       [lhs_hi]  [lhs_lo]
    //                     X___________________
    //                       [------tmp0------]
    //             [------tmp1------]
    //             [------tmp2------]
    //     [------tmp3------]
    //                       [-------add------]
    // +_______________________________________
    //                       [------sum0------]
    //     [------sum1------]

    let lhs_lo = lhs as u64;
    let rhs_lo = rhs as u64;
    let lhs_hi = (lhs.wrapping_shr(64)) as u64;
    let rhs_hi = (rhs.wrapping_shr(64)) as u64;
    let tmp0 = (lhs_lo as u128).wrapping_mul(rhs_lo as u128);
    let tmp1 = (lhs_lo as u128).wrapping_mul(rhs_hi as u128);
    let tmp2 = (lhs_hi as u128).wrapping_mul(rhs_lo as u128);
    let tmp3 = (lhs_hi as u128).wrapping_mul(rhs_hi as u128);
    // tmp1 and tmp2 straddle the boundary. We have to handle three carries
    let (sum0, carry0) = tmp0.overflowing_add(tmp1.wrapping_shl(64));
    let (sum0, carry1) = sum0.overflowing_add(tmp2.wrapping_shl(64));
    let (sum0, carry2) = sum0.overflowing_add(add as u128);
    let sum1 = tmp3
        .wrapping_add(tmp1.wrapping_shr(64))
        .wrapping_add(tmp2.wrapping_shr(64))
        .wrapping_add(carry0 as u128)
        .wrapping_add(carry1 as u128)
        .wrapping_add(carry2 as u128);
    (sum0, sum1)
}
