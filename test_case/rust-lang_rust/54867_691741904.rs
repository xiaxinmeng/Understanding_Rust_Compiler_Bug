
pub fn u128_div_rem_10(duo: u128) -> (u128, u128) {
    let duo_hi = (duo >> 64) as u64;
    let div_0 = 10;
    let (quo_hi, rem_3) = (duo_hi / div_0, duo_hi % div_0);

    let duo_mid =
        ((duo >> 32) as u32 as u64)
        | (rem_3 << 32);
    let (quo_1, rem_2) = (duo_mid / div_0, duo_mid % div_0);

    let duo_lo =
        (duo as u32 as u64)
        | (rem_2 << 32);
    let (quo_0, rem_1) = (duo_lo / div_0, duo_lo % div_0);

    return (
        (quo_0 as u128)
        | ((quo_1 as u128) << 32)
        | ((quo_hi as u128) << 64),
        rem_1 as u128
    )
}

