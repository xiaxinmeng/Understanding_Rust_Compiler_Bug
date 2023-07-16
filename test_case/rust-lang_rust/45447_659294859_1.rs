
error[E0401]: can't use generic parameters from outer function
  --> rle/src/encode.rs:15:27
   |
14 | pub fn encode_rle_to<T: UnsignedInt>(input: &[T], output: &mut impl Output<T>) -> Result<()> {
   |                      - type parameter from outer function
15 |   const MAX_LEN: usize = (T::MAX).into() >> 1;
   |                           ^^^^^^ use of generic parameter from outer function
