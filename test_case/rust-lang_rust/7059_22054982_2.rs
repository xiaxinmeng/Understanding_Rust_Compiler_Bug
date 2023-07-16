 rust
  #[priority=2]
  pub fn abs<T: Signed>(value: T) -> T { value.abs() }
  #[priority=1]
  pub fn abs<T: Ord + Zero + Neg<T>>(v: T) -> T { if v < Zero::zero() { v.neg() } else { v } }
  