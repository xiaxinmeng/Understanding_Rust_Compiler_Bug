
trait Bounded: Sized {
  const fn max_value() -> Self;
  const fn min_value() -> Self;
}

impl Bounded for i8 { ... }
...
impl Bounded for u128 { ... }
