rust
fn my_u128_checked_mul(left: u128, right: u128) -> Option<u128> {
    if right == 0 {
        return Some(0);
    }

    // we have left * right
    // we write left * (right_a + right_b), where right_a is the highest power of two.
    let highest_set_bit = 127 - right.leading_zeros();
    let right_a = (1 as u128) << highest_set_bit;
    let right_b = right - right_a;

    // check if left * right_a overflows.
    // we use that left * right_a == left << highest_set_bit
    if left.leading_zeros() < highest_set_bit  {
        return None
    }
    // compute (left * right_a) + (left * right_b)
    // since right_a > right_b, left * right_b is guaranteed not to overflow.
    left.shl(highest_set_bit).checked_add(left * right_b)
}
