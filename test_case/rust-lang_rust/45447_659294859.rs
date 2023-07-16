
pub fn encode_rle_to<T: UnsignedInt>(input: &[T], output: &mut impl Output<T>) -> Result<()> {
  const MAX_LEN: usize = (T::MAX).into() >> 1;
