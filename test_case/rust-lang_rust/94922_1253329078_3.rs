rust
type T = usize;

// f(x, y) = (x + y ⋅ 2ⁿ) ÷ 2 = x ÷ 2 + y ⋅ 2^(n - 1)
fn divide_by_2_with_carry(
 x: T,
 carry: bool
) -> T {
 (x >> 1) | (T::from(carry) << (T::BITS - 1))
}

// (cneg(x, y), (0 != x) & y)
fn overflowing_cneg(
 x: T,
 y: bool
) -> (T, bool) {
 if y {
  x.overflowing_neg()
 } else {
  (x, false)
 }
}

fn cneg(x: T, y: bool) -> T {
 if y {
  x.wrapping_neg()
 } else {
  x
 }
}

// sum(x, y) = [x ≤ y] ⋅ (x + y) ⋅ (y - x + 1) ÷ 2
pub fn sum(x: T, y: T) -> T {
 T::from(x <= y) * {
  let is_odd = 1 == 1 & (x ^ y);
  let a = {
   let (result, underflowed) = overflowing_cneg(x, is_odd);
   let (result, overflowed_1) = result.overflowing_add(y);
   let (result, overflowed_2) = result.overflowing_add(T::from(is_odd));
   let carry_bit = underflowed ^ overflowed_1 ^ overflowed_2;
   let result = divide_by_2_with_carry(
    result,
    carry_bit
   );
   result
  };
  let b = cneg(x, !is_odd) + y + T::from(!is_odd);
  a * b
 }
}
