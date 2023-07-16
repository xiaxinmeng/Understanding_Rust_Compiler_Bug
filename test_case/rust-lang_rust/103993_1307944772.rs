plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
┐rustc_mir_transform::inline::inline body=num::<impl usize>::from_ne_bytes
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::is_nan
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::abs_private
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::is_infinite
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::is_finite
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::is_subnormal
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::is_normal
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::is_sign_positive
├─┐rustc_mir_transform::inline::inline body=f32::<impl f32>::is_sign_negative
┘
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::recip
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::to_degrees
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::to_radians
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::max
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::min
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::maximum
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::minimum
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::to_int_unchecked
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::to_bits
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::from_bits
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::to_be_bytes
├─┐rustc_mir_transform::inline::inline body=num::<impl u32>::to_be_bytes
│ ├─┐rustc_mir_transform::inline::inline body=num::<impl u32>::to_be
│ │ ├─┐rustc_mir_transform::inline::inline body=num::<impl u32>::swap_bytes
│ ├─┘
│ ├─┘
│ ├─┐rustc_mir_transform::inline::inline body=num::<impl u32>::to_ne_bytes
├─┘
┘
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::to_le_bytes
├─┐rustc_mir_transform::inline::inline body=num::<impl u32>::to_le_bytes
│ ├─┐rustc_mir_transform::inline::inline body=num::<impl u32>::to_le
├─┘
┘
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::to_ne_bytes
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::from_be_bytes
├─┐rustc_mir_transform::inline::inline body=num::<impl u32>::from_be_bytes
│ ├─┐rustc_mir_transform::inline::inline body=num::<impl u32>::from_ne_bytes
│ ├─┘
│ ├─┐rustc_mir_transform::inline::inline body=num::<impl u32>::from_be
├─┘
┘
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::from_le_bytes
├─┐rustc_mir_transform::inline::inline body=num::<impl u32>::from_le_bytes
│ ├─┐rustc_mir_transform::inline::inline body=num::<impl u32>::from_le
├─┘
┘
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::from_ne_bytes
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::total_cmp
├─┐rustc_mir_transform::inline::inline body=cmp::impls::<impl cmp::Ord for i32>::cmp
┘
┘
┐rustc_mir_transform::inline::inline body=f32::<impl f32>::clamp
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::is_nan
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::abs_private
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::is_infinite
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::is_finite
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::is_subnormal
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::is_normal
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::is_sign_positive
├─┐rustc_mir_transform::inline::inline body=f64::<impl f64>::is_sign_negative
┘
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::is_positive
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::is_negative
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::recip
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::to_degrees
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::to_radians
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::max
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::min
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::maximum
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::minimum
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::to_int_unchecked
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::to_bits
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::from_bits
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::to_be_bytes
├─┐rustc_mir_transform::inline::inline body=num::<impl u64>::to_be_bytes
│ ├─┐rustc_mir_transform::inline::inline body=num::<impl u64>::to_be
│ │ ├─┐rustc_mir_transform::inline::inline body=num::<impl u64>::swap_bytes
│ ├─┘
│ ├─┘
│ ├─┐rustc_mir_transform::inline::inline body=num::<impl u64>::to_ne_bytes
├─┘
┘
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::to_le_bytes
├─┐rustc_mir_transform::inline::inline body=num::<impl u64>::to_le_bytes
│ ├─┐rustc_mir_transform::inline::inline body=num::<impl u64>::to_le
├─┘
┘
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::to_ne_bytes
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::from_be_bytes
├─┐rustc_mir_transform::inline::inline body=num::<impl u64>::from_be_bytes
│ ├─┐rustc_mir_transform::inline::inline body=num::<impl u64>::from_ne_bytes
│ ├─┘
│ ├─┐rustc_mir_transform::inline::inline body=num::<impl u64>::from_be
├─┘
┘
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::from_le_bytes
├─┐rustc_mir_transform::inline::inline body=num::<impl u64>::from_le_bytes
│ ├─┐rustc_mir_transform::inline::inline body=num::<impl u64>::from_le
├─┘
┘
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::from_ne_bytes
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::total_cmp
├─┐rustc_mir_transform::inline::inline body=cmp::impls::<impl cmp::Ord for i64>::cmp
┘
┘
┐rustc_mir_transform::inline::inline body=f64::<impl f64>::clamp
┘
┐rustc_mir_transform::inline::inline body=Big32x40::is_zero::{closure#0}
┘
┐rustc_mir_transform::inline::inline body=Big32x40::bit_length::{closure#0}
┘
┐rustc_mir_transform::inline::inline body=Big32x40::mul_pow2::{closure#0}
┘
┐rustc_mir_transform::inline::inline body=Big32x40::div_rem::{closure#0}
┘
┐rustc_mir_transform::inline::inline body=Big32x40::div_rem::{closure#1}
┘
┐rustc_mir_transform::inline::inline body=Big8x3::is_zero::{closure#0}
┘
┐rustc_mir_transform::inline::inline body=Big8x3::bit_length::{closure#0}
┘
┐rustc_mir_transform::inline::inline body=Big8x3::mul_pow2::{closure#0}
┘
┐rustc_mir_transform::inline::inline body=Big8x3::div_rem::{closure#0}
┘
┐rustc_mir_transform::inline::inline body=Big8x3::div_rem::{closure#1}
┘
┐rustc_mir_transform::inline::inline body=<Option<T> as cmp::PartialEq>::eq
┘
┐rustc_mir_transform::inline::inline body=num::<impl u8>::is_ascii_digit
┘
┐rustc_mir_transform::inline::inline body=<AsciiStr<'a> as cmp::PartialEq>::eq
├─┐rustc_mir_transform::inline::inline body=cmp::impls::<impl cmp::PartialEq<&B> for &A>::eq
┘
┘
┐rustc_mir_transform::inline::inline body=num::<impl u8>::wrapping_sub
┘
┐rustc_mir_transform::inline::inline body=<BiasedFp as Clone>::clone
┘
┐rustc_mir_transform::inline::inline body=<BiasedFp as cmp::PartialEq>::eq
┘
┐rustc_mir_transform::inline::inline body=<BiasedFp as cmp::Eq>::assert_receiver_is_total_eq
┘
┐rustc_mir_transform::inline::inline body=<BiasedFp as Default>::default
├─┐rustc_mir_transform::inline::inline body=<u64 as Default>::default
├─┘
├─┐rustc_mir_transform::inline::inline body=<i32 as Default>::default
┘
┘
┐rustc_mir_transform::inline::inline body=<Decimal as Clone>::clone
├─┐rustc_mir_transform::inline::inline body=clone::impls::<impl Clone for usize>::clone
├─┘
├─┐rustc_mir_transform::inline::inline body=clone::impls::<impl Clone for i32>::clone
├─┘
├─┐rustc_mir_transform::inline::inline body=clone::impls::<impl Clone for bool>::clone
┘
┘
┐rustc_mir_transform::inline::inline body=num::<impl u64>::leading_zeros
┘
┐rustc_mir_transform::inline::inline body=<Number as Clone>::clone
┘
┐rustc_mir_transform::inline::inline body=<Number as Default>::default
├─┐rustc_mir_transform::inline::inline body=<i64 as Default>::default
├─┘
├─┐rustc_mir_transform::inline::inline body=<bool as Default>::default
┘
┘
┐rustc_mir_transform::inline::inline body=<Number as cmp::PartialEq>::eq
┘
┐rustc_mir_transform::inline::inline body=<Number as cmp::Eq>::assert_receiver_is_total_eq
┘
┐rustc_mir_transform::inline::inline body=Number::is_fast_path
┘
┐rustc_mir_transform::inline::inline body=Number::try_fast_path
├─┐rustc_mir_transform::inline::inline body=num::<impl u64>::checked_mul
│ ├─┐rustc_mir_transform::inline::inline body=num::<impl u64>::overflowing_mul
├─┘
├─┘
├─┐rustc_mir_transform::inline::inline body=<Option<T> as FromResidual>::from_residual
┘
┘
┐rustc_mir_transform::inline::inline body=num::<impl u64>::wrapping_mul
┘
┐rustc_mir_transform::inline::inline body=num::<impl u64>::wrapping_add
┘
┐rustc_mir_transform::inline::inline body=dec2flt::<impl FromStr for f32>::from_str
┘
┐rustc_mir_transform::inline::inline body=dec2flt::<impl FromStr for f64>::from_str
┘
┐rustc_mir_transform::inline::inline body=<ParseFloatError as Clone>::clone
├─┐rustc_mir_transform::inline::inline body=<FloatErrorKind as Clone>::clone
┘
┘
┐rustc_mir_transform::inline::inline body=<ParseFloatError as cmp::PartialEq>::eq
├─┐rustc_mir_transform::inline::inline body=<FloatErrorKind as cmp::PartialEq>::eq
┘
┘
┐rustc_mir_transform::inline::inline body=<ParseFloatError as cmp::Eq>::assert_receiver_is_total_eq
┘
┐rustc_mir_transform::inline::inline body=<FloatErrorKind as cmp::Eq>::assert_receiver_is_total_eq
┘
┐rustc_mir_transform::inline::inline body=str::<impl str>::as_bytes
┘
┐rustc_mir_transform::inline::inline body=<Fp as Clone>::clone
┘
┐rustc_mir_transform::inline::inline body=<Decoded as Clone>::clone
┘
┐rustc_mir_transform::inline::inline body=<Decoded as cmp::PartialEq>::eq
┘
┐rustc_mir_transform::inline::inline body=<Decoded as cmp::Eq>::assert_receiver_is_total_eq
┘
┐rustc_mir_transform::inline::inline body=<FullDecoded as Clone>::clone
┘
┐rustc_mir_transform::inline::inline body=<FullDecoded as cmp::PartialEq>::eq
┘
┐rustc_mir_transform::inline::inline body=<FullDecoded as cmp::Eq>::assert_receiver_is_total_eq
┘
┐rustc_mir_transform::inline::inline body=<Sign as Clone>::clone
┘
┐rustc_mir_transform::inline::inline body=<Sign as cmp::PartialEq>::eq
┘
┐rustc_mir_transform::inline::inline body=<Sign as cmp::Eq>::assert_receiver_is_total_eq
┘
┐rustc_mir_transform::inline::inline body=<Part<'a> as cmp::PartialEq>::eq
┘
┐rustc_mir_transform::inline::inline body=<TryFromIntError as Clone>::clone
┘
┐rustc_mir_transform::inline::inline body=<TryFromIntError as cmp::PartialEq>::eq
├─┐rustc_mir_transform::inline::inline body=cmp::impls::<impl cmp::PartialEq for ()>::eq
┘
┘
┐rustc_mir_transform::inline::inline body=<TryFromIntError as cmp::Eq>::assert_receiver_is_total_eq
┘
┐rustc_mir_transform::inline::inline body=<ParseIntError as Clone>::clone
├─┐rustc_mir_transform::inline::inline body=<IntErrorKind as Clone>::clone
┘
┘
┐rustc_mir_transform::inline::inline body=<ParseIntError as cmp::PartialEq>::eq
├─┐rustc_mir_transform::inline::inline body=<IntErrorKind as cmp::PartialEq>::eq
┘
┘
┐rustc_mir_transform::inline::inline body=<ParseIntError as cmp::Eq>::assert_receiver_is_total_eq
┘
┐rustc_mir_transform::inline::inline body=<IntErrorKind as cmp::Eq>::assert_receiver_is_total_eq
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU8 as Clone>::clone
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU8 as cmp::Eq>::assert_receiver_is_total_eq
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU8 as cmp::PartialEq>::eq
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU8 as cmp::Ord>::cmp
├─┐rustc_mir_transform::inline::inline body=cmp::impls::<impl cmp::Ord for u8>::cmp
┘
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU8 as cmp::PartialOrd>::partial_cmp
├─┐rustc_mir_transform::inline::inline body=cmp::impls::<impl cmp::PartialOrd for u8>::partial_cmp
┘
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU8 as hash::Hash>::hash
├─┐rustc_mir_transform::inline::inline body=hash::impls::<impl hash::Hash for u8>::hash
┘
┘
┐rustc_mir_transform::inline::inline body=nonzero::NonZeroU8::new_unchecked
├─┐rustc_mir_transform::inline::inline body=nonzero::NonZeroU8::new_unchecked::runtime
┘
┘
┐rustc_mir_transform::inline::inline body=nonzero::NonZeroU8::new
┘
┐rustc_mir_transform::inline::inline body=nonzero::NonZeroU8::get
┘
┐rustc_mir_transform::inline::inline body=nonzero::<impl From<nonzero::NonZeroU8> for u8>::from
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU8 as bit::BitOr>::bitor
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU8 as bit::BitOr<u8>>::bitor
┘
┐rustc_mir_transform::inline::inline body=nonzero::<impl bit::BitOr<nonzero::NonZeroU8> for u8>::bitor
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU8 as BitOrAssign>::bitor_assign
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU8 as BitOrAssign<u8>>::bitor_assign
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU8 as fmt::Debug>::fmt
├─┐rustc_mir_transform::inline::inline body=fmt::num::<impl fmt::Debug for u8>::fmt
┘
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU8 as Display>::fmt
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU8 as fmt::Binary>::fmt
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU8 as fmt::Octal>::fmt
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU8 as fmt::LowerHex>::fmt
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU8 as fmt::UpperHex>::fmt
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU16 as Clone>::clone
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU16 as cmp::Eq>::assert_receiver_is_total_eq
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU16 as cmp::PartialEq>::eq
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU16 as cmp::Ord>::cmp
├─┐rustc_mir_transform::inline::inline body=cmp::impls::<impl cmp::Ord for u16>::cmp
┘
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU16 as cmp::PartialOrd>::partial_cmp
├─┐rustc_mir_transform::inline::inline body=cmp::impls::<impl cmp::PartialOrd for u16>::partial_cmp
┘
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU16 as hash::Hash>::hash
├─┐rustc_mir_transform::inline::inline body=hash::impls::<impl hash::Hash for u16>::hash
┘
┘
┐rustc_mir_transform::inline::inline body=nonzero::NonZeroU16::new_unchecked
├─┐rustc_mir_transform::inline::inline body=nonzero::NonZeroU16::new_unchecked::runtime
┘
┘
┐rustc_mir_transform::inline::inline body=nonzero::NonZeroU16::new
┘
┐rustc_mir_transform::inline::inline body=nonzero::NonZeroU16::get
┘
┐rustc_mir_transform::inline::inline body=nonzero::<impl From<nonzero::NonZeroU16> for u16>::from
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU16 as bit::BitOr>::bitor
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU16 as bit::BitOr<u16>>::bitor
┘
┐rustc_mir_transform::inline::inline body=nonzero::<impl bit::BitOr<nonzero::NonZeroU16> for u16>::bitor
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU16 as BitOrAssign>::bitor_assign
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU16 as BitOrAssign<u16>>::bitor_assign
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU16 as fmt::Debug>::fmt
├─┐rustc_mir_transform::inline::inline body=fmt::num::<impl fmt::Debug for u16>::fmt
┘
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU16 as Display>::fmt
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU16 as fmt::Binary>::fmt
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU16 as fmt::Octal>::fmt
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU16 as fmt::LowerHex>::fmt
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU16 as fmt::UpperHex>::fmt
┘
┐rustc_mir_transform::inline::inline body=<NonZeroU32 as Clone>::clone
┘
┐rustc_mir_transform::inline::inline body=<NonZeroU32 as cmp::Eq>::assert_receiver_is_total_eq
┘
┐rustc_mir_transform::inline::inline body=<NonZeroU32 as cmp::PartialEq>::eq
┘
┐rustc_mir_transform::inline::inline body=<NonZeroU32 as cmp::Ord>::cmp
├─┐rustc_mir_transform::inline::inline body=cmp::impls::<impl cmp::Ord for u32>::cmp
┘
┘
┐rustc_mir_transform::inline::inline body=<NonZeroU32 as cmp::PartialOrd>::partial_cmp
├─┐rustc_mir_transform::inline::inline body=cmp::impls::<impl cmp::PartialOrd for u32>::partial_cmp
┘
┘
┐rustc_mir_transform::inline::inline body=<NonZeroU32 as hash::Hash>::hash
├─┐rustc_mir_transform::inline::inline body=hash::impls::<impl hash::Hash for u32>::hash
┘
┘
┐rustc_mir_transform::inline::inline body=NonZeroU32::new_unchecked
├─┐rustc_mir_transform::inline::inline body=NonZeroU32::new_unchecked::runtime
┘
┘
┐rustc_mir_transform::inline::inline body=NonZeroU32::new
┘
┐rustc_mir_transform::inline::inline body=NonZeroU32::get
┘
┐rustc_mir_transform::inline::inline body=nonzero::<impl From<NonZeroU32> for u32>::from
┘
┐rustc_mir_transform::inline::inline body=<NonZeroU32 as bit::BitOr>::bitor
┘
┐rustc_mir_transform::inline::inline body=<NonZeroU32 as bit::BitOr<u32>>::bitor
┘
┐rustc_mir_transform::inline::inline body=nonzero::<impl bit::BitOr<NonZeroU32> for u32>::bitor
┘
┐rustc_mir_transform::inline::inline body=<NonZeroU32 as BitOrAssign>::bitor_assign
┘
┐rustc_mir_transform::inline::inline body=<NonZeroU32 as BitOrAssign<u32>>::bitor_assign
┘
┐rustc_mir_transform::inline::inline body=<NonZeroU32 as fmt::Debug>::fmt
├─┐rustc_mir_transform::inline::inline body=fmt::num::<impl fmt::Debug for u32>::fmt
┘
┘
┐rustc_mir_transform::inline::inline body=<NonZeroU32 as Display>::fmt
┘
┐rustc_mir_transform::inline::inline body=<NonZeroU32 as fmt::Binary>::fmt
┘
┐rustc_mir_transform::inline::inline body=<NonZeroU32 as fmt::Octal>::fmt
┘
┐rustc_mir_transform::inline::inline body=<NonZeroU32 as fmt::LowerHex>::fmt
┘
┐rustc_mir_transform::inline::inline body=<NonZeroU32 as fmt::UpperHex>::fmt
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU64 as Clone>::clone
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU64 as cmp::Eq>::assert_receiver_is_total_eq
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU64 as cmp::PartialEq>::eq
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU64 as cmp::Ord>::cmp
├─┐rustc_mir_transform::inline::inline body=cmp::impls::<impl cmp::Ord for u64>::cmp
┘
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU64 as cmp::PartialOrd>::partial_cmp
├─┐rustc_mir_transform::inline::inline body=cmp::impls::<impl cmp::PartialOrd for u64>::partial_cmp
┘
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU64 as hash::Hash>::hash
├─┐rustc_mir_transform::inline::inline body=hash::impls::<impl hash::Hash for u64>::hash
┘
┘
┐rustc_mir_transform::inline::inline body=nonzero::NonZeroU64::new_unchecked
├─┐rustc_mir_transform::inline::inline body=nonzero::NonZeroU64::new_unchecked::runtime
┘
┘
┐rustc_mir_transform::inline::inline body=nonzero::NonZeroU64::new
┘
┐rustc_mir_transform::inline::inline body=nonzero::NonZeroU64::get
┘
┐rustc_mir_transform::inline::inline body=nonzero::<impl From<nonzero::NonZeroU64> for u64>::from
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU64 as bit::BitOr>::bitor
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU64 as bit::BitOr<u64>>::bitor
┘
┐rustc_mir_transform::inline::inline body=nonzero::<impl bit::BitOr<nonzero::NonZeroU64> for u64>::bitor
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU64 as BitOrAssign>::bitor_assign
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU64 as BitOrAssign<u64>>::bitor_assign
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU64 as fmt::Debug>::fmt
├─┐rustc_mir_transform::inline::inline body=fmt::num::<impl fmt::Debug for u64>::fmt
┘
┘
┐rustc_mir_transform::inline::inline body=<nonzero::NonZeroU64 as Display>::fmt
┘
---
            29: core::fmt::Formatter::debug_struct_field3_finish
            30: <&rustc_middle::mir::BasicBlockData as core::fmt::Debug>::fmt
            31: core::fmt::builders::DebugInner::entry
            32: core::fmt::builders::DebugSet::entry
            33: <core::fmt::builders::DebugList>::entries::<&rustc_middle::mir::BasicBlockData, core::slice::iter::Iter<rustc_middle::mir::BasicBlockData>>
            34: <alloc::vec::Vec<rustc_middle::mir::BasicBlockData> as core::fmt::Debug>::fmt
            36: core::fmt::Formatter::debug_struct_field5_finish
            36: core::fmt::Formatter::debug_struct_field5_finish
            37: <&rustc_middle::mir::basic_blocks::BasicBlocks as core::fmt::Debug>::fmt
            39: core::fmt::Formatter::debug_struct_fields_finish
            39: core::fmt::Formatter::debug_struct_fields_finish
            40: <&&rustc_middle::mir::Body as core::fmt::Debug>::fmt
            42: <alloc::string::String as core::fmt::Write>::write_fmt
            42: <alloc::string::String as core::fmt::Write>::write_fmt
            43: <tracing_subscriber::fmt::format::DefaultVisitor as tracing_core::field::Visit>::record_debug
            44: <tracing_core::field::ValueSet>::record
            45: <tracing_subscriber::fmt::format::DefaultFields as tracing_subscriber::fmt::format::FormatFields>::format_fields::<&tracing_core::span::Attributes>
            46: <tracing_error::layer::ErrorLayer<tracing_subscriber::registry::sharded::Registry> as tracing_subscriber::layer::Layer<tracing_subscriber::registry::sharded::Registry>>::on_new_span
            47: <tracing_subscriber::filter::layer_filters::FilterState>::did_enable::<<tracing_subscriber::filter::layer_filters::Filtered<tracing_error::layer::ErrorLayer<tracing_subscriber::registry::sharded::Registry>, tracing_subscriber::filter::filter_fn::FilterFn<rustc_log::init_env_logger::{closure#0}>, tracing_subscriber::registry::sharded::Registry> as tracing_subscriber::layer::Layer<tracing_subscriber::registry::sharded::Registry>>::on_new_span::{closure#0}>
            48: <std::thread::local::LocalKey<tracing_subscriber::filter::layer_filters::FilterState>>::with::<<tracing_subscriber::filter::layer_filters::Filtered<tracing_error::layer::ErrorLayer<tracing_subscriber::registry::sharded::Registry>, tracing_subscriber::filter::filter_fn::FilterFn<rustc_log::init_env_logger::{closure#0}>, tracing_subscriber::registry::sharded::Registry>>::did_enable<<tracing_subscriber::filter::layer_filters::Filtered<tracing_error::layer::ErrorLayer<tracing_subscriber::registry::sharded::Registry>, tracing_subscriber::filter::filter_fn::FilterFn<rustc_log::init_env_logger::{closure#0}>, tracing_subscriber::registry::sharded::Registry> as tracing_subscriber::layer::Layer<tracing_subscriber::registry::sharded::Registry>>::on_new_span::{closure#0}>::{closure#0}, ()>
            49: <tracing_subscriber::filter::layer_filters::Filtered<tracing_error::layer::ErrorLayer<tracing_subscriber::registry::sharded::Registry>, tracing_subscriber::filter::filter_fn::FilterFn<rustc_log::init_env_logger::{closure#0}>, tracing_subscriber::registry::sharded::Registry> as tracing_subscriber::layer::Layer<tracing_subscriber::registry::sharded::Registry>>::on_new_span
            50: <tracing_subscriber::layer::layered::Layered<tracing_subscriber::filter::layer_filters::Filtered<tracing_tree::HierarchicalLayer<std::io::stdio::stderr>, tracing_subscriber::filter::env::EnvFilter, tracing_subscriber::layer::layered::Layered<tracing_subscriber::filter::layer_filters::Filtered<tracing_error::layer::ErrorLayer<tracing_subscriber::registry::sharded::Registry>, tracing_subscriber::filter::filter_fn::FilterFn<rustc_log::init_env_logger::{closure#0}>, tracing_subscriber::registry::sharded::Registry>, tracing_subscriber::registry::sharded::Registry>>, tracing_subscriber::layer::layered::Layered<tracing_subscriber::filter::layer_filters::Filtered<tracing_error::layer::ErrorLayer<tracing_subscriber::registry::sharded::Registry>, tracing_subscriber::filter::filter_fn::FilterFn<rustc_log::init_env_logger::{closure#0}>, tracing_subscriber::registry::sharded::Registry>, tracing_subscriber::registry::sharded::Registry>> as tracing_core::subscriber::Subscriber>::new_span
            51: <tracing::span::Span>::make_with
            52: tracing_core::dispatcher::get_default::<tracing::span::Span, <tracing::span::Span>::new::{closure#0}>
            53: <tracing::span::Span>::new
            54: <rustc_middle::mir::Body as rustc_middle::ty::visit::TypeVisitable>::has_non_region_param
            55: <rustc_middle::mir::Body>::new
            57: rustc_mir_build::build::mir_built
            57: rustc_mir_build::build::mir_built
            58: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_middle::ty::WithOptConstParam<rustc_span::def_id::LocalDefId>, &rustc_data_structures::steal::Steal<rustc_middle::mir::Body>>>
            59: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::mir_built, rustc_query_impl::plumbing::QueryCtxt>
            61: rustc_mir_transform::check_unsafety::unsafety_check_result
            61: rustc_mir_transform::check_unsafety::unsafety_check_result
            62: <rustc_mir_transform::check_unsafety::provide::{closure#0} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId)>>::call_once
            63: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::mir::query::UnsafetyCheckResult>>
            64: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::unsafety_check_result, rustc_query_impl::plumbing::QueryCtxt>
            65: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::unsafety_check_result
            66: <rustc_mir_transform::check_unsafety::UnsafetyChecker as rustc_middle::mir::visit::Visitor>::visit_rvalue
            67: rustc_mir_transform::check_unsafety::unsafety_check_result
            68: <rustc_mir_transform::check_unsafety::provide::{closure#0} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId)>>::call_once
            69: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::mir::query::UnsafetyCheckResult>>
            70: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::unsafety_check_result, rustc_query_impl::plumbing::QueryCtxt>
            71: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::unsafety_check_result
            72: rustc_mir_transform::mir_const
            73: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_middle::ty::WithOptConstParam<rustc_span::def_id::LocalDefId>, &rustc_data_structures::steal::Steal<rustc_middle::mir::Body>>>
            75: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_const
            76: rustc_mir_transform::mir_promoted
            77: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::mir_promoted, rustc_query_impl::plumbing::QueryCtxt>
            78: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_promoted
            78: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_promoted
            79: rustc_borrowck::mir_borrowck
            80: <rustc_borrowck::provide::{closure#0} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId)>>::call_once
            81: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::mir::query::BorrowCheckResult>>
            82: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::mir_borrowck, rustc_query_impl::plumbing::QueryCtxt>
            84: rustc_hir_analysis::collect::type_of::find_opaque_ty_constraints_for_rpit
            85: rustc_hir_analysis::collect::type_of::type_of
            86: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::ty::Ty>>
            87: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::type_of, rustc_query_impl::plumbing::QueryCtxt>
            87: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::type_of, rustc_query_impl::plumbing::QueryCtxt>
            88: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::type_of
            89: <rustc_privacy::ReachEverythingInTheInterfaceVisitor>::ty
            90: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_item
            91: rustc_hir::intravisit::walk_ty::<rustc_privacy::EmbargoVisitor>
            92: rustc_hir::intravisit::walk_fn::<rustc_privacy::EmbargoVisitor>
            93: rustc_hir::intravisit::walk_item::<rustc_privacy::EmbargoVisitor>
            94: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_item
            95: rustc_hir::intravisit::walk_item::<rustc_privacy::EmbargoVisitor>
            96: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_item
            97: rustc_hir::intravisit::walk_item::<rustc_privacy::EmbargoVisitor>
            98: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_item
            99: rustc_hir::intravisit::walk_item::<rustc_privacy::EmbargoVisitor>
           100: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_item
           101: rustc_hir::intravisit::walk_mod::<rustc_privacy::EmbargoVisitor>
           102: rustc_privacy::effective_visibilities
           103: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), &rustc_middle::middle::privacy::EffectiveVisibilities>>
           104: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::effective_visibilities, rustc_query_impl::plumbing::QueryCtxt>
           105: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::effective_visibilities
           106: rustc_passes::stability::check_unused_or_stable_features
           107: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
           108: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#0}::{closure#2}>, ()>
           109: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#0}>
           111: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
           112: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
           113: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
           114: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
           114: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
           115: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
           116: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
           117: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
           118: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
           119: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
           120: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
           122: <unknown>
           123: <unknown>
          


thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1585:13
stack backtrace:
   0:     0x7f3e123e4572 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h751d52596fbe295b
   1:     0x7f3e12452308 - core::fmt::write::h6d6e96066401bc0f
   2:     0x7f3e123d4da1 - std::io::Write::write_fmt::h9fe4e6d9c9b927ef
   3:     0x7f3e123e4335 - std::sys_common::backtrace::print::hf38d1633e21dbba9
   4:     0x7f3e123e7717 - std::panicking::default_hook::{{closure}}::h12022b1a20dd35ce
   5:     0x7f3e123e7475 - std::panicking::default_hook::h5ab5b9712723f5dd
   6:     0x7f3e12d6cdf4 - rustc_driver[ced6e23715cad135]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f3e123e8023 - std::panicking::rust_panic_with_hook::h57cd9b8bb3f6a82b
   8:     0x7f3e15adcd93 - std[389b380b19480123]::panicking::begin_panic::<rustc_errors[33a73980a86ec781]::ExplicitBug>::{closure#0}
   9:     0x7f3e15adbf86 - std[389b380b19480123]::sys_common::backtrace::__rust_end_short_backtrace::<std[389b380b19480123]::panicking::begin_panic<rustc_errors[33a73980a86ec781]::ExplicitBug>::{closure#0}, !>
  10:     0x7f3e12d398a6 - std[389b380b19480123]::panicking::begin_panic::<rustc_errors[33a73980a86ec781]::ExplicitBug>
  11:     0x7f3e15acef96 - std[389b380b19480123]::panic::panic_any::<rustc_errors[33a73980a86ec781]::ExplicitBug>
  12:     0x7f3e15ad3d55 - <rustc_errors[33a73980a86ec781]::HandlerInner as core[69c2305d6fa5d54f]::ops::drop::Drop>::drop
  13:     0x7f3e12d87e62 - core[69c2305d6fa5d54f]::ptr::drop_in_place::<rustc_session[5fc19af3c98d6915]::parse::ParseSess>
  14:     0x7f3e12d89208 - core[69c2305d6fa5d54f]::ptr::drop_in_place::<rustc_session[5fc19af3c98d6915]::session::Session>
  15:     0x7f3e12d91d33 - <alloc[a40b22d2678a71d4]::rc::Rc<rustc_session[5fc19af3c98d6915]::session::Session> as core[69c2305d6fa5d54f]::ops::drop::Drop>::drop
  16:     0x7f3e12d7608c - core[69c2305d6fa5d54f]::ptr::drop_in_place::<rustc_interface[a1fc70c15f43b5ea]::interface::Compiler>
  17:     0x7f3e12d6e679 - rustc_span[5b068acac2dfc73a]::with_source_map::<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[33a73980a86ec781]::ErrorGuaranteed>, rustc_interface[a1fc70c15f43b5ea]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[33a73980a86ec781]::ErrorGuaranteed>, rustc_driver[ced6e23715cad135]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  18:     0x7f3e12dcd41c - <scoped_tls[ce9fa241ba16890b]::ScopedKey<rustc_span[5b068acac2dfc73a]::SessionGlobals>>::set::<rustc_interface[a1fc70c15f43b5ea]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[33a73980a86ec781]::ErrorGuaranteed>, rustc_driver[ced6e23715cad135]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[33a73980a86ec781]::ErrorGuaranteed>>
  19:     0x7f3e12d8caaa - std[389b380b19480123]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a1fc70c15f43b5ea]::util::run_in_thread_pool_with_globals<rustc_interface[a1fc70c15f43b5ea]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[33a73980a86ec781]::ErrorGuaranteed>, rustc_driver[ced6e23715cad135]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[33a73980a86ec781]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[33a73980a86ec781]::ErrorGuaranteed>>
  20:     0x7f3e12dce446 - std[389b380b19480123]::panic::catch_unwind::<core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<<std[389b380b19480123]::thread::Builder>::spawn_unchecked_<rustc_interface[a1fc70c15f43b5ea]::util::run_in_thread_pool_with_globals<rustc_interface[a1fc70c15f43b5ea]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[33a73980a86ec781]::ErrorGuaranteed>, rustc_driver[ced6e23715cad135]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[33a73980a86ec781]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[33a73980a86ec781]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[33a73980a86ec781]::ErrorGuaranteed>>
  21:     0x7f3e12d7e3c9 - <<std[389b380b19480123]::thread::Builder>::spawn_unchecked_<rustc_interface[a1fc70c15f43b5ea]::util::run_in_thread_pool_with_globals<rustc_interface[a1fc70c15f43b5ea]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[33a73980a86ec781]::ErrorGuaranteed>, rustc_driver[ced6e23715cad135]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[33a73980a86ec781]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[33a73980a86ec781]::ErrorGuaranteed>>::{closure#1} as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:     0x7f3e123f4c8e - std::sys::unix::thread::Thread::new::thread_start::hb7aff98f3d211bd8
  23:     0x7f3e1218ab43 - <unknown>
  24:     0x7f3e1221ca00 - <unknown>
  25:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (2e19fac72 2022-11-08) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
SpanTrace:

error: could not compile `core`
┐rustc_mir_transform::inline::inline body=<float::cmp::Result as Clone>::clone
┘
┐rustc_mir_transform::inline::inline body=float::cmp::Result::to_le_abi
┘
┐rustc_mir_transform::inline::inline body=float::cmp::Result::to_ge_abi
┘
┐rustc_mir_transform::inline::inline body=<f32 as Float>::from_repr
┘
┐rustc_mir_transform::inline::inline body=<f32 as Float>::from_parts
┘
┐rustc_mir_transform::inline::inline body=<f32 as Float>::normalize
┘
┐rustc_mir_transform::inline::inline body=<f64 as Float>::from_repr
┘
┐rustc_mir_transform::inline::inline body=<f64 as Float>::from_parts
┘
┐rustc_mir_transform::inline::inline body=<f64 as Float>::normalize
┘
┐rustc_mir_transform::inline::inline body=<u16 as DInt>::from_lo_hi
┘
┐rustc_mir_transform::inline::inline body=<u32 as DInt>::from_lo_hi
┘
┐rustc_mir_transform::inline::inline body=<u64 as DInt>::from_lo_hi
┘
┐rustc_mir_transform::inline::inline body=<u128 as DInt>::from_lo_hi
┘
┐rustc_mir_transform::inline::inline body=<i16 as DInt>::from_lo_hi
┘
┐rustc_mir_transform::inline::inline body=<i32 as DInt>::from_lo_hi
┘
┐rustc_mir_transform::inline::inline body=<i64 as DInt>::from_lo_hi
┘
┐rustc_mir_transform::inline::inline body=<i128 as DInt>::from_lo_hi
┘
┐rustc_mir_transform::inline::inline body=<u32 as CastInto<u32>>::cast
┘
┐rustc_mir_transform::inline::inline body=<f32 as Float>::repr
┘
┐rustc_mir_transform::inline::inline body=<u32 as Int>::wrapping_sub
┘
┐rustc_mir_transform::inline::inline body=<u32 as CastInto<i32>>::cast
┘
┐rustc_mir_transform::inline::inline body=<i32 as CastInto<u32>>::cast
┘
┐rustc_mir_transform::inline::inline body=<u32 as Int>::from_bool
┘
┐rustc_mir_transform::inline::inline body=<u32 as Int>::leading_zeros
┘
┐rustc_mir_transform::inline::inline body=<u32 as CastInto<u64>>::cast
┘
┐rustc_mir_transform::inline::inline body=<f64 as Float>::repr
┘
┐rustc_mir_transform::inline::inline body=<u64 as Int>::wrapping_sub
┘
┐rustc_mir_transform::inline::inline body=<u64 as CastInto<i32>>::cast
┘
┐rustc_mir_transform::inline::inline body=<i32 as CastInto<u64>>::cast
┘
┐rustc_mir_transform::inline::inline body=<u64 as CastInto<u32>>::cast
┘
┐rustc_mir_transform::inline::inline body=<u64 as Int>::from_bool
┘
┐rustc_mir_transform::inline::inline body=<u64 as Int>::leading_zeros
┘
┐rustc_mir_transform::inline::inline body=<f32 as Float>::signed_repr
┘
┐rustc_mir_transform::inline::inline body=<f64 as Float>::signed_repr
┘
┐rustc_mir_transform::inline::inline body=<u32 as HInt>::widen_mul
┘
┐rustc_mir_transform::inline::inline body=<u32 as HInt>::widen
┘
┐rustc_mir_transform::inline::inline body=<u64 as DInt>::hi
┘
┐rustc_mir_transform::inline::inline body=<u32 as Int>::wrapping_mul
┘
┐rustc_mir_transform::inline::inline body=<u32 as Int>::wrapping_add
┘
┐rustc_mir_transform::inline::inline body=<u64 as CastInto<u64>>::cast
┘
┐rustc_mir_transform::inline::inline body=<u64 as HInt>::widen_mul
┘
┐rustc_mir_transform::inline::inline body=<u64 as HInt>::widen
┘
┐rustc_mir_transform::inline::inline body=<u128 as DInt>::hi
┘
┐rustc_mir_transform::inline::inline body=<u64 as Int>::wrapping_mul
┘
┐rustc_mir_transform::inline::inline body=<u64 as Int>::wrapping_add
┘
┐rustc_mir_transform::inline::inline body=<u64 as Int>::wrapping_shl
┘
┐rustc_mir_transform::inline::inline body=<u64 as DInt>::lo_hi
┘
┐rustc_mir_transform::inline::inline body=<u64 as DInt>::lo
┘
┐rustc_mir_transform::inline::inline body=<u128 as DInt>::lo_hi
┘
┐rustc_mir_transform::inline::inline body=<u128 as DInt>::lo
┘
┐rustc_mir_transform::inline::inline body=<i32 as Int>::abs_diff
┘
┐rustc_mir_transform::inline::inline body=<u64 as Int>::wrapping_shr
┘
┐rustc_mir_transform::inline::inline body=<i128 as Int>::unsigned
┘
┐rustc_mir_transform::inline::inline body=<u64 as Int>::overflowing_add
┘
┐rustc_mir_transform::inline::inline body=<u64 as HInt>::zero_widen
┘
┐rustc_mir_transform::inline::inline body=<u64 as HInt>::widen_hi
┘
┐rustc_mir_transform::inline::inline body=<i128 as Int>::from_unsigned
┘
┐rustc_mir_transform::inline::inline body=<u128 as Int>::unsigned
┘
┐rustc_mir_transform::inline::inline body=<u128 as Int>::from_unsigned
┘
┐rustc_mir_transform::inline::inline body=<u32 as DInt>::lo
┘
┐rustc_mir_transform::inline::inline body=<u16 as HInt>::zero_widen_mul
┘
┐rustc_mir_transform::inline::inline body=<u16 as HInt>::zero_widen
┘
┐rustc_mir_transform::inline::inline body=<u32 as DInt>::hi
┘
┐rustc_mir_transform::inline::inline body=<u32 as HInt>::zero_widen
┘
┐rustc_mir_transform::inline::inline body=<u32 as HInt>::widen_hi
┘
┐rustc_mir_transform::inline::inline body=<i128 as DInt>::lo
┘
┐rustc_mir_transform::inline::inline body=<i64 as DInt>::lo
┘
┐rustc_mir_transform::inline::inline body=<i32 as HInt>::zero_widen_mul
┘
┐rustc_mir_transform::inline::inline body=<i32 as HInt>::zero_widen
┘
┐rustc_mir_transform::inline::inline body=<i64 as DInt>::hi
┘
┐rustc_mir_transform::inline::inline body=<i64 as HInt>::zero_widen
┘
┐rustc_mir_transform::inline::inline body=<i64 as HInt>::widen_hi
┘
┐rustc_mir_transform::inline::inline body=<i128 as Int>::wrapping_add
┘
┐rustc_mir_transform::inline::inline body=<i128 as DInt>::hi
┘
┐rustc_mir_transform::inline::inline body=<i64 as Int>::wrapping_mul
┘
┐rustc_mir_transform::inline::inline body=<u16 as Int>::is_zero
┘
┐rustc_mir_transform::inline::inline body=<u16 as HInt>::widen_mul
┘
┐rustc_mir_transform::inline::inline body=<u16 as HInt>::widen
┘
┐rustc_mir_transform::inline::inline body=<u16 as HInt>::widen_hi
┘
┐rustc_mir_transform::inline::inline body=<u32 as Int>::overflowing_add
┘
┐rustc_mir_transform::inline::inline body=<u32 as Int>::is_zero
┘
┐rustc_mir_transform::inline::inline body=<u64 as Int>::is_zero
┘
┐rustc_mir_transform::inline::inline body=<u128 as Int>::wrapping_mul
┘
┐rustc_mir_transform::inline::inline body=<u128 as Int>::overflowing_add
┘
┐rustc_mir_transform::inline::inline body=<u16 as Int>::wrapping_shl
┘
┐rustc_mir_transform::inline::inline body=<u16 as Int>::logical_shr
┘
┐rustc_mir_transform::inline::inline body=<u16 as Int>::unsigned
┘
┐rustc_mir_transform::inline::inline body=<u16 as Int>::from_unsigned
┘
┐rustc_mir_transform::inline::inline body=<u32 as Int>::wrapping_shl
┘
┐rustc_mir_transform::inline::inline body=<u32 as Int>::logical_shr
┘
┐rustc_mir_transform::inline::inline body=<u32 as Int>::unsigned
┘
┐rustc_mir_transform::inline::inline body=<u32 as Int>::from_unsigned
┘
┐rustc_mir_transform::inline::inline body=<u64 as Int>::logical_shr
┘
┐rustc_mir_transform::inline::inline body=<u64 as Int>::unsigned
┘
┐rustc_mir_transform::inline::inline body=<u64 as Int>::from_unsigned
┘
┐rustc_mir_transform::inline::inline body=<i32 as DInt>::hi
┘
┐rustc_mir_transform::inline::inline body=<i16 as Int>::wrapping_shr
┘
┐rustc_mir_transform::inline::inline body=<i16 as HInt>::zero_widen
┘
┐rustc_mir_transform::inline::inline body=<i16 as HInt>::widen_hi
┘
┐rustc_mir_transform::inline::inline body=<i32 as DInt>::lo
┘
┐rustc_mir_transform::inline::inline body=<i16 as Int>::logical_shr
┘
┐rustc_mir_transform::inline::inline body=<i16 as Int>::unsigned
┘
┐rustc_mir_transform::inline::inline body=<i16 as Int>::from_unsigned
┘
┐rustc_mir_transform::inline::inline body=<i16 as Int>::wrapping_shl
┘
┐rustc_mir_transform::inline::inline body=<i32 as Int>::wrapping_shr
┘
┐rustc_mir_transform::inline::inline body=<i32 as HInt>::widen_hi
┘
┐rustc_mir_transform::inline::inline body=<i32 as Int>::logical_shr
┘
┐rustc_mir_transform::inline::inline body=<i32 as Int>::unsigned
┘
┐rustc_mir_transform::inline::inline body=<i32 as Int>::from_unsigned
┘
┐rustc_mir_transform::inline::inline body=<i32 as Int>::wrapping_shl
┘
┐rustc_mir_transform::inline::inline body=<i64 as Int>::wrapping_shr
┘
┐rustc_mir_transform::inline::inline body=<i64 as Int>::logical_shr
┘
┐rustc_mir_transform::inline::inline body=<i64 as Int>::unsigned
┘
┐rustc_mir_transform::inline::inline body=<i64 as Int>::from_unsigned
┘
┐rustc_mir_transform::inline::inline body=<i64 as Int>::wrapping_shl
┘
┐rustc_mir_transform::inline::inline body=<f32 as Float>::eq_repr
┘
┐rustc_mir_transform::inline::inline body=<f32 as Float>::sign
┘
┐rustc_mir_transform::inline::inline body=<f32 as Float>::exp
┘
┐rustc_mir_transform::inline::inline body=<f32 as Float>::frac
┘
┐rustc_mir_transform::inline::inline body=<f32 as Float>::imp_frac
┘
┐rustc_mir_transform::inline::inline body=<f32 as Float>::is_subnormal
┘
┐rustc_mir_transform::inline::inline body=<f64 as Float>::eq_repr
┘
┐rustc_mir_transform::inline::inline body=<f64 as Float>::sign
┘
┐rustc_mir_transform::inline::inline body=<f64 as Float>::exp
┘
┐rustc_mir_transform::inline::inline body=<f64 as Float>::frac
┘
┐rustc_mir_transform::inline::inline body=<f64 as Float>::imp_frac
┘
┐rustc_mir_transform::inline::inline body=<f64 as Float>::is_subnormal
┘
┐rustc_mir_transform::inline::inline body=<usize as Int>::unsigned
┘
┐rustc_mir_transform::inline::inline body=<usize as Int>::from_unsigned
┘
┐rustc_mir_transform::inline::inline body=<usize as Int>::abs_diff
┘
┐rustc_mir_transform::inline::inline body=<usize as Int>::from_bool
┘
┐rustc_mir_transform::inline::inline body=<usize as Int>::logical_shr
┘
┐rustc_mir_transform::inline::inline body=<usize as Int>::is_zero
┘
┐rustc_mir_transform::inline::inline body=<usize as Int>::wrapping_neg
┘
┐rustc_mir_transform::inline::inline body=<usize as Int>::wrapping_add
┘
┐rustc_mir_transform::inline::inline body=<usize as Int>::wrapping_mul
┘
┐rustc_mir_transform::inline::inline body=<usize as Int>::wrapping_sub
┘
┐rustc_mir_transform::inline::inline body=<usize as Int>::wrapping_shl
┘
┐rustc_mir_transform::inline::inline body=<usize as Int>::wrapping_shr
┘
┐rustc_mir_transform::inline::inline body=<usize as Int>::rotate_left
┘
┐rustc_mir_transform::inline::inline body=<usize as Int>::overflowing_add
┘
┐rustc_mir_transform::inline::inline body=<usize as Int>::leading_zeros
┘
┐rustc_mir_transform::inline::inline body=<isize as Int>::unsigned
┘
┐rustc_mir_transform::inline::inline body=<isize as Int>::from_unsigned
┘
┐rustc_mir_transform::inline::inline body=<isize as Int>::abs_diff
┘
┐rustc_mir_transform::inline::inline body=<isize as Int>::from_bool
┘
┐rustc_mir_transform::inline::inline body=<isize as Int>::logical_shr
┘
┐rustc_mir_transform::inline::inline body=<isize as Int>::is_zero
┘
┐rustc_mir_transform::inline::inline body=<isize as Int>::wrapping_neg
┘
┐rustc_mir_transform::inline::inline body=<isize as Int>::wrapping_add
┘
┐rustc_mir_transform::inline::inline body=<isize as Int>::wrapping_mul
┘
┐rustc_mir_transform::inline::inline body=<isize as Int>::wrapping_sub
┘
┐rustc_mir_transform::inline::inline body=<isize as Int>::wrapping_shl
┘
┐rustc_mir_transform::inline::inline body=<isize as Int>::wrapping_shr
┘
┐rustc_mir_transform::inline::inline body=<isize as Int>::rotate_left
┘
┐rustc_mir_transform::inline::inline body=<isize as Int>::overflowing_add
┘
┐rustc_mir_transform::inline::inline body=<isize as Int>::leading_zeros
┘
┐rustc_mir_transform::inline::inline body=<u8 as Int>::unsigned
┘
┐rustc_mir_transform::inline::inline body=<u8 as Int>::from_unsigned
┘
┐rustc_mir_transform::inline::inline body=<u8 as Int>::abs_diff
┘
┐rustc_mir_transform::inline::inline body=<u8 as Int>::from_bool
┘
┐rustc_mir_transform::inline::inline body=<u8 as Int>::logical_shr
┘
┐rustc_mir_transform::inline::inline body=<u8 as Int>::is_zero
┘
┐rustc_mir_transform::inline::inline body=<u8 as Int>::wrapping_neg
┘
┐rustc_mir_transform::inline::inline body=<u8 as Int>::wrapping_add
┘
┐rustc_mir_transform::inline::inline body=<u8 as Int>::wrapping_mul
┘
┐rustc_mir_transform::inline::inline body=<u8 as Int>::wrapping_sub
┘
┐rustc_mir_transform::inline::inline body=<u8 as Int>::wrapping_shl
┘
┐rustc_mir_transform::inline::inline body=<u8 as Int>::wrapping_shr
┘
┐rustc_mir_transform::inline::inline body=<u8 as Int>::rotate_left
┘
┐rustc_mir_transform::inline::inline body=<u8 as Int>::overflowing_add
┘
┐rustc_mir_transform::inline::inline body=<u8 as Int>::leading_zeros
┘
┐rustc_mir_transform::inline::inline body=<i8 as Int>::unsigned
┘
┐rustc_mir_transform::inline::inline body=<i8 as Int>::from_unsigned
┘
┐rustc_mir_transform::inline::inline body=<i8 as Int>::abs_diff
┘
┐rustc_mir_transform::inline::inline body=<i8 as Int>::from_bool
┘
┐rustc_mir_transform::inline::inline body=<i8 as Int>::logical_shr
┘
┐rustc_mir_transform::inline::inline body=<i8 as Int>::is_zero
┘
┐rustc_mir_transform::inline::inline body=<i8 as Int>::wrapping_neg
┘
┐rustc_mir_transform::inline::inline body=<i8 as Int>::wrapping_add
