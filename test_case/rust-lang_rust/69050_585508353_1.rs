
          .           macro_rules! impl_write_unsigned_leb128 {
          .               ($fn_name:ident, $int_ty:ident) => {
          .                   #[inline]
          .                   pub fn $fn_name(out: &mut Vec<u8>, mut value: $int_ty) {
          .                       for _ in 0..leb128_size!($int_ty) {
143,877,210 ( 1.61%)                  let mut byte = (value & 0x7F) as u8;
 48,003,612 ( 0.54%)                  value >>= 7;
239,884,434 ( 2.69%)                  if value != 0 {
 47,959,070 ( 0.54%)                      byte |= 0x80;
          .                           }
          .
          .                           write_to_vec(out, byte);
          .
 47,959,070 ( 0.54%)                  if value == 0 {
          .                               break;
          .                           }
          .                       }
          .                   }
          .               };
          .           }
          .
          .           impl_write_unsigned_leb128!(write_u16_leb128, u16);
-- line 50 ----------------------------------------
-- line 57 ----------------------------------------
          .               ($fn_name:ident, $int_ty:ident) => {
          .                   #[inline]
          .                   pub fn $fn_name(slice: &[u8]) -> ($int_ty, usize) {
          .                       let mut result: $int_ty = 0;
          .                       let mut shift = 0;
          .                       let mut position = 0;
          .
          .                       for _ in 0..leb128_size!($int_ty) {
 59,507,824 ( 0.67%)                  let byte = unsafe { *slice.get_unchecked(position) };
          .                           position += 1;
204,126,888 ( 2.29%)                  result |= ((byte & 0x7F) as $int_ty) << shift;
119,023,350 ( 1.33%)                  if (byte & 0x80) == 0 {
          .                               break;
          .                           }
          .                           shift += 7;
          .                       }
          .
          .                       // Do a single bounds check at the end instead of for every byte.
 67,805,748 ( 0.76%)              assert!(position <= slice.len());
          .
          .                       (result, position)
          .                   }
          .               };
          .           }
