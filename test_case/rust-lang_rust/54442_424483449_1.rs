
trait SizeEquivalence {
  type Signed;
  type Unsigned;
}

impl SizeEquivalence for u8 {
  type Signed = i8;
  type Unsigned = u8;
}
...
