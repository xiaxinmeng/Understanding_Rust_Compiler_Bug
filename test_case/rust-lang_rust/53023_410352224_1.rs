rust
fn my_u128_checked_mul_2(left: u128, right: u128) -> Option<u128> {
    let left_bits = 128 - left.leading_zeros();
    let right_bits = 128 - right.leading_zeros();
    if left_bits + right_bits > 128 {
        None
    } else {
        Some(left * right)
    }
}
