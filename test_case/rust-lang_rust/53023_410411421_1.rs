rust
trait TryCastToI128 {
    fn try_cast_to_i128(self) -> Option<i128>;

    fn try_cast_to_neg_i128(self) -> Option<i128>;
}

impl TryCastToI128 for u128 {
    fn try_cast_to_i128(self) -> Option<i128> {
        if self > (i128::max_value() as u128) {
            None
        } else {
            Some(self as i128)
        }
    }

    fn try_cast_to_neg_i128(self) -> Option<i128> {
        if self == (i128::min_value() as u128) {
            Some(i128::min_value())
        } else if self > (i128::max_value() as u128) {
            None
        } else {
            Some(-(self as i128))
        }
    }
}

pub fn my_i128_checked_mul(left: i128, right: i128) -> Option<i128> {
    if left >= 0 && right >= 0 {
        my_u128_checked_mul_3(left as u128, right as u128)?.try_cast_to_i128()
    } else if left >= 0 && right < 0 {
        if left == 0 {
            Some(0)
        } else if left == 1 {
            Some(right)
        } else if right == i128::min_value() {
            None // always overflow
        } else {
            my_u128_checked_mul_3(left as u128, -right as u128)?.try_cast_to_neg_i128()
        }
    } else if left < 0 && right >= 0 {
        if right == 0 {
            Some(0)
        } else if right == 1 {
            Some(left)
        } else if left == i128::min_value() {
            None // always overflow
        } else {
            my_u128_checked_mul_3(-left as u128, right as u128)?.try_cast_to_neg_i128()
        }
    } else {
        // left < 0 && right < 0
        if left == i128::min_value() {
            None
        } else if right == i128::min_value() {
            None
        } else {
            my_u128_checked_mul_3(-left as u128, -right as u128)?.try_cast_to_i128()
        }
    }
}
