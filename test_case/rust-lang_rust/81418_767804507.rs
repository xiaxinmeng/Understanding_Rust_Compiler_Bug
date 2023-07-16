plain
   Compiling libc v0.2.79
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: macro expansion ignores token `#` and any following
   |
   |
26 |         #[doc = concat!(
   | 
  ::: library/core/src/num/shells/i128.rs:13:1
   |
   |
13 | int_module! { i128, #[stable(feature = "i128", since="1.26.0")] }
   | ----------------------------------------------------------------- caused by the macro expansion here
   |
   = note: the usage of `int_module!` is likely invalid in item context

error: macro expansion ignores token `#` and any following
   |
   |
4  |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   |                    --------------------------------------------------------------- caused by the macro expansion here
...
26 |         #[doc = concat!(
   |
   |
   = note: the usage of `int_module!` is likely invalid in item context

error: macro expansion ignores token `#` and any following
   |
   |
4  |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   |                    --------------------------------------------------------------- caused by the macro expansion here
...
26 |         #[doc = concat!(
   |
   |
   = note: the usage of `int_module!` is likely invalid in item context

error: macro expansion ignores token `#` and any following
   |
   |
4  |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   |                    --------------------------------------------------------------- caused by the macro expansion here
...
26 |         #[doc = concat!(
   |
   |
   = note: the usage of `int_module!` is likely invalid in item context

error: macro expansion ignores token `#` and any following
   |
   |
4  |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   |                    --------------------------------------------------------------- caused by the macro expansion here
...
26 |         #[doc = concat!(
   |
   |
   = note: the usage of `int_module!` is likely invalid in item context

error: macro expansion ignores token `#` and any following
   |
   |
4  |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   |                    --------------------------------------------------------------- caused by the macro expansion here
...
26 |         #[doc = concat!(
   |
   |
   = note: the usage of `int_module!` is likely invalid in item context

error: macro expansion ignores token `#` and any following
   |
   |
26 |         #[doc = concat!(
   | 
  ::: library/core/src/num/shells/u128.rs:13:1
   |
   |
13 | int_module! { u128, #[stable(feature = "i128", since="1.26.0")] }
   | ----------------------------------------------------------------- caused by the macro expansion here
   |
   = note: the usage of `int_module!` is likely invalid in item context

error: macro expansion ignores token `#` and any following
   |
   |
4  |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   |                    --------------------------------------------------------------- caused by the macro expansion here
...
26 |         #[doc = concat!(
   |
   |
   = note: the usage of `int_module!` is likely invalid in item context

error: macro expansion ignores token `#` and any following
   |
   |
4  |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   |                    --------------------------------------------------------------- caused by the macro expansion here
...
26 |         #[doc = concat!(
   |
   |
   = note: the usage of `int_module!` is likely invalid in item context

error: macro expansion ignores token `#` and any following
   |
   |
4  |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   |                    --------------------------------------------------------------- caused by the macro expansion here
...
26 |         #[doc = concat!(
   |
   |
   = note: the usage of `int_module!` is likely invalid in item context

error: macro expansion ignores token `#` and any following
   |
   |
4  |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   |                    --------------------------------------------------------------- caused by the macro expansion here
...
26 |         #[doc = concat!(
   |
   |
   = note: the usage of `int_module!` is likely invalid in item context

error: macro expansion ignores token `#` and any following
   |
   |
4  |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   |                    --------------------------------------------------------------- caused by the macro expansion here
...
26 |         #[doc = concat!(
   |
   |
   = note: the usage of `int_module!` is likely invalid in item context

error: macro expansion ignores token `impl` and any following
  --> library/core/src/num/bignum.rs:47:13
   |
47 |               impl FullOps for $ty {
...
...
84 | / impl_full_ops! {
85 | |     u8:  add(intrinsics::u8_add_with_overflow),  mul/div(u16);
86 | |     u16: add(intrinsics::u16_add_with_overflow), mul/div(u32);
87 | |     u32: add(intrinsics::u32_add_with_overflow), mul/div(u64);
88 | |     // See RFC #521 for enabling this.
89 | |     // u64: add(intrinsics::u64_add_with_overflow), mul/div(u128);
90 | | }
   | |_- caused by the macro expansion here
   |
   = note: the usage of `impl_full_ops!` is likely invalid in item context

error: macro expansion ignores token `impl` and any following
   --> library/core/src/num/bignum.rs:117:9
117 |         impl $name {
    |         ^^^^
...
...
467 | define_bignum!(Big32x40: type=Digit32, n=40);
    | --------------------------------------------- caused by the macro expansion here
    |
    = note: the usage of `define_bignum!` is likely invalid in item context

error: macro expansion ignores token `impl` and any following
   --> library/core/src/num/bignum.rs:117:9
117 |         impl $name {
    |         ^^^^
...
...
472 |     define_bignum!(Big8x3: type=u8, n=3);
    |     ------------------------------------- caused by the macro expansion here
    |
    = note: the usage of `define_bignum!` is likely invalid in item context

error: macro expansion ignores token `const` and any following
   --> library/core/src/num/dec2flt/rawfp.rs:131:9
    |
131 |         const MAX_EXP: i16 = (1 << (Self::EXP_BITS - 1)) - 1;
...
...
154 |     other_constants!(f32);
    |     ---------------------- caused by the macro expansion here
    |
    = note: the usage of `other_constants!` is likely invalid in impl item context

error: macro expansion ignores token `const` and any following
   --> library/core/src/num/dec2flt/rawfp.rs:131:9
    |
131 |         const MAX_EXP: i16 = (1 << (Self::EXP_BITS - 1)) - 1;
...
...
203 |     other_constants!(f64);
    |     ---------------------- caused by the macro expansion here
    |
    = note: the usage of `other_constants!` is likely invalid in impl item context

error: macro expansion ignores token `impl` and any following
    |
    |
44  |               impl $Ty {
...
...
148 | / nonzero_integers! {
149 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
150 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
151 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
...   |
160 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
161 | | }
    | |_- caused by the macro expansion here
    |
    = note: the usage of `nonzero_integers!` is likely invalid in item context

error: macro expansion ignores token `#` and any following
    |
    |
165 |           #[stable(feature = "nonzero_parse", since = "1.35.0")]
...
...
178 | / from_str_radix_nzint_impl! { NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize
179 | | NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize }
    | |_____________________________________________________________________- caused by the macro expansion here
    |
    = note: the usage of `from_str_radix_nzint_impl!` is likely invalid in item context

error: macro expansion ignores token `impl` and any following
    |
    |
184 |               impl $Ty {
...
...
235 | / nonzero_leading_trailing_zeros! {
236 | |     NonZeroU8(u8), u8::MAX;
237 | |     NonZeroU16(u16), u16::MAX;
238 | |     NonZeroU32(u32), u32::MAX;
...   |
247 | |     NonZeroIsize(usize), -1isize;
248 | | }
    | |_- caused by the macro expansion here
    |
    = note: the usage of `nonzero_leading_trailing_zeros!` is likely invalid in item context

error: macro expansion ignores token `#` and any following
    |
    |
266 |               #[stable(feature = "nonzero_div", since = "1.51.0")]
...
...
281 | / nonzero_integers_div! {
282 | |     NonZeroU8(u8);
283 | |     NonZeroU16(u16);
284 | |     NonZeroU32(u32);
287 | |     NonZeroUsize(usize);
288 | | }
288 | | }
    | |_- caused by the macro expansion here
    |
    = note: the usage of `nonzero_integers_div!` is likely invalid in item context

error: macro expansion ignores token `impl` and any following
    |
    |
293 |             impl $Ty {
...
...
328 | nonzero_unsigned_is_power_of_two! { NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize }
    | --------------------------------------------------------------------------------------------------------- caused by the macro expansion here
    |
    = note: the usage of `nonzero_unsigned_is_power_of_two!` is likely invalid in item context

error: macro expansion ignores token `sh_impl_unsigned` and any following
    |
    |
191 |         sh_impl_unsigned! { $t, usize }
...
...
202 | sh_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
    | -------------------------------------------------------------------- caused by the macro expansion here
    |
    = note: the usage of `sh_impl_all!` is likely invalid in item context

error: macro expansion ignores token `forward_ref_binop` and any following
    |
    |
148 |         forward_ref_binop! { impl Shl, shl for Wrapping<$t>, $f,
...
...
191 |         sh_impl_unsigned! { $t, usize }
    |         ------------------------------- caused by the macro expansion here
    |
    = note: the usage of `sh_impl_unsigned!` is likely invalid in item context

error: macro expansion ignores token `forward_ref_binop` and any following
    |
    |
216 |         forward_ref_binop! { impl Add, add for Wrapping<$t>, Wrapping<$t>,
...
...
401 | wrapping_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
    | ---------------------------------------------------------------------- caused by the macro expansion here
    |
    = note: the usage of `wrapping_impl!` is likely invalid in item context

error: macro expansion ignores token `impl` and any following
    |
    |
405 |         impl Wrapping<$t> {
...
...
739 | wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
    | -------------------------------------------------------------------------- caused by the macro expansion here
    |
    = note: the usage of `wrapping_int_impl!` is likely invalid in item context

error: macro expansion ignores token `impl` and any following
    |
    |
743 |         impl Wrapping<$t> {
...
...
857 | wrapping_int_impl_signed! { isize i8 i16 i32 i64 i128 }
    | ------------------------------------------------------- caused by the macro expansion here
    |
    = note: the usage of `wrapping_int_impl_signed!` is likely invalid in item context

error: macro expansion ignores token `impl` and any following
    |
    |
861 |         impl Wrapping<$t> {
...
...
928 | wrapping_int_impl_unsigned! { usize u8 u16 u32 u64 u128 }
    | --------------------------------------------------------- caused by the macro expansion here
    |
    = note: the usage of `wrapping_int_impl_unsigned!` is likely invalid in item context

error: macro expansion ignores token `/// The largest value that can be represented by this integer type.` and any following
   |
18 |           /// The largest value that can be represented by this integer type.
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   | 
   | 
  ::: library/core/src/num/mod.rs:91:5
   |
91 | /     int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
92 | |     "[0x12]", "[0x12]", "", "" }
   | |________________________________- caused by the macro expansion here
   |
   = note: the usage of `int_impl!` is likely invalid in impl item context

error: macro expansion ignores token `/// The largest value that can be represented by this integer type.` and any following
   |
18 |           /// The largest value that can be represented by this integer type.
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   | 
   | 
  ::: library/core/src/num/mod.rs:97:5
   |
97 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
98 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
   | |______________________________________________________- caused by the macro expansion here
   |
   = note: the usage of `int_impl!` is likely invalid in impl item context

error: macro expansion ignores token `/// The largest value that can be represented by this integer type.` and any following
    |
18  |           /// The largest value that can be represented by this integer type.
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    | 
    | 
   ::: library/core/src/num/mod.rs:103:5
    |
103 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
104 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
105 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
    | |________________________________________- caused by the macro expansion here
    |
    = note: the usage of `int_impl!` is likely invalid in impl item context

error: macro expansion ignores token `/// The largest value that can be represented by this integer type.` and any following
    |
18  |           /// The largest value that can be represented by this integer type.
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    | 
    | 
   ::: library/core/src/num/mod.rs:110:5
    |
110 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, 12,
111 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
112 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
113 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
    | |________________________________________________________________- caused by the macro expansion here
    |
    = note: the usage of `int_impl!` is likely invalid in impl item context

error: macro expansion ignores token `/// The largest value that can be represented by this integer type.` and any following
    |
18  |           /// The largest value that can be represented by this integer type.
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    | 
    | 
   ::: library/core/src/num/mod.rs:118:5
    |
118 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
119 | |     170141183460469231731687303715884105727, 16,
120 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
121 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...   |
124 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
125 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
    | |________________________________________________________________- caused by the macro expansion here
    |
    = note: the usage of `int_impl!` is likely invalid in impl item context

error: macro expansion ignores token `/// The largest value that can be represented by this integer type.` and any following
    |
18  |           /// The largest value that can be represented by this integer type.
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    | 
    | 
   ::: library/core/src/num/mod.rs:148:5
    |
148 | /     int_impl! { isize, i64, usize, 64, -9223372036854775808, 9223372036854775807,
149 | |     12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
150 | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
151 | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
152 | |      usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
    | |_______________________________________________________________________- caused by the macro expansion here
    |
    = note: the usage of `int_impl!` is likely invalid in impl item context

error: macro expansion ignores token `/// The largest value that can be represented by this integer type.` and any following
   --> library/core/src/num/uint_macros.rs:18:9
18  |           /// The largest value that can be represented by this integer type.
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    | 
   ::: library/core/src/num/mod.rs:157:5
   ::: library/core/src/num/mod.rs:157:5
    |
157 | /     uint_impl! { u8, u8, 8, 255, 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
158 | |     "[0x12]", "", "" }
    | |______________________- caused by the macro expansion here
    |
    = note: the usage of `uint_impl!` is likely invalid in impl item context

error: macro expansion ignores token `/// The largest value that can be represented by this integer type.` and any following
   --> library/core/src/num/uint_macros.rs:18:9
18  |           /// The largest value that can be represented by this integer type.
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    | 
   ::: library/core/src/num/mod.rs:656:5
   ::: library/core/src/num/mod.rs:656:5
    |
656 | /     uint_impl! { u16, u16, 16, 65535, 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
657 | |     "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
    | |____________________________________________- caused by the macro expansion here
    |
    = note: the usage of `uint_impl!` is likely invalid in impl item context

error: macro expansion ignores token `/// The largest value that can be represented by this integer type.` and any following
   --> library/core/src/num/uint_macros.rs:18:9
18  |           /// The largest value that can be represented by this integer type.
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    | 
   ::: library/core/src/num/mod.rs:662:5
   ::: library/core/src/num/mod.rs:662:5
    |
662 | /     uint_impl! { u32, u32, 32, 4294967295, 8, "0x10000b3", "0xb301", "0x12345678",
663 | |     "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }
    | |________________________________________________________________________________________________- caused by the macro expansion here
    |
    = note: the usage of `uint_impl!` is likely invalid in impl item context

error: macro expansion ignores token `/// The largest value that can be represented by this integer type.` and any following
   --> library/core/src/num/uint_macros.rs:18:9
18  |           /// The largest value that can be represented by this integer type.
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    | 
   ::: library/core/src/num/mod.rs:668:5
   ::: library/core/src/num/mod.rs:668:5
    |
668 | /     uint_impl! { u64, u64, 64, 18446744073709551615, 12, "0xaa00000000006e1", "0x6e10aa",
669 | |     "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
670 | |     "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
671 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
672 | |     "", ""}
    | |___________- caused by the macro expansion here
    |
    = note: the usage of `uint_impl!` is likely invalid in impl item context

error: macro expansion ignores token `/// The largest value that can be represented by this integer type.` and any following
   --> library/core/src/num/uint_macros.rs:18:9
18  |           /// The largest value that can be represented by this integer type.
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    | 
---
   |
10 | use crate::cmp;
   |     ^^^^^^^^^^

error: unused imports: `Add`, `Sub`
  |
  |
4 | use crate::ops::{self, Add, Sub, Try};
  |                        ^^^  ^^^
error: unused import: `crate::num::Wrapping`
 --> library/core/src/iter/traits/accum.rs:2:5
  |
2 | use crate::num::Wrapping;
2 | use crate::num::Wrapping;
  |     ^^^^^^^^^^^^^^^^^^^^

error: unused import: `Mul`
 --> library/core/src/iter/traits/accum.rs:3:23
  |
3 | use crate::ops::{Add, Mul};


error: unused imports: `Display`, `LowerExp`, `UpperExp`
 --> library/core/src/fmt/float.rs:1:25
  |
1 | use crate::fmt::{Debug, Display, Formatter, LowerExp, Result, UpperExp};
  |                         ^^^^^^^             ^^^^^^^^          ^^^^^^^^
error: unused macro definition
    --> library/core/src/fmt/mod.rs:2169:1
     |
     |
2169 | / macro_rules! peel {
2170 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
     | |_^

error: unused macro definition
  --> library/core/src/slice/iter/macros.rs:4:1
  --> library/core/src/slice/iter/macros.rs:4:1
   |
4  | / macro_rules! is_empty {
5  | |     // The way we encode the length of a ZST iterator, this works both for ZST
6  | |     // and non-ZST.
7  | |     ($self: ident) => {
8  | |         $self.ptr.as_ptr() as *const T == $self.end
10 | | }
   | |_^

error: unused import: `crate::cmp::Ordering`
---

error: unused import: `Sized`
  --> library/core/src/slice/iter.rs:12:40
   |
12 | use crate::marker::{PhantomData, Send, Sized, Sync};

error: unused macro definition
   --> library/core/src/slice/iter/macros.rs:52:9
    |
    |
41  |   / macro_rules! iterator {
42  |   |     (
43  |   |         struct $name:ident -> $ptr:ty,
44  |   |         $elem:ty,
...     |
52  | / |         macro_rules! next_unchecked {
53  | | |             ($self: ident) => {& $( $mut_ )? *$self.post_inc_start(1)}
54  | | |         }
    | |_|_________^
381 |   |     }
382 |   | }
382 |   | }
    |   |_- in this expansion of `iterator!`
   ::: library/core/src/slice/iter.rs:134:1
    |
    |
134 | /   iterator! {struct Iter -> *const T, &'a T, const, {/* no mut */}, {
135 | |       fn is_sorted_by<F>(self, mut compare: F) -> bool
136 | |       where
137 | |           Self: Sized,
143 | |       }
144 | |   }}
    | |____- in this macro invocation


error: unused macro definition
   --> library/core/src/slice/iter/macros.rs:52:9
    |
41  |  / macro_rules! iterator {
42  |  |     (
43  |  |         struct $name:ident -> $ptr:ty,
44  |  |         $elem:ty,
...    |
52  | /|         macro_rules! next_unchecked {
53  | ||             ($self: ident) => {& $( $mut_ )? *$self.post_inc_start(1)}
54  | ||         }
    | ||_________^
381 |  |     }
382 |  | }
382 |  | }
    |  |_- in this expansion of `iterator!`
   ::: library/core/src/slice/iter.rs:308:1
    |
    |
308 |    iterator! {struct IterMut -> *mut T, &'a mut T, mut, {mut}, {}}

error: unused import: `crate::ops::Try`
 --> library/core/src/str/iter.rs:8:5
  |
  |
8 | use crate::ops::Try;
  |     ^^^^^^^^^^^^^^^

error: unused import: `DoubleEndedSearcher`
   |
   |
14 | use super::pattern::{DoubleEndedSearcher, ReverseSearcher, Searcher};


error: unused import: `crate::cmp::Ordering::*`
 --> library/core/src/tuple.rs:3:5
3 | use crate::cmp::Ordering::*;
  |     ^^^^^^^^^^^^^^^^^^^^^^^

error: unused macro definition
error: unused macro definition
  --> library/core/src/tuple.rs:77:1
   |
77 | / macro_rules! lexical_ord {
78 | |     ($rel: ident, $a:expr, $b:expr, $($rest_a:expr, $rest_b:expr),+) => {
79 | |         if $a != $b { lexical_ord!($rel, $a, $b) }
80 | |         else { lexical_ord!($rel, $($rest_a, $rest_b),+) }
81 | |     };
82 | |     ($rel: ident, $a:expr, $b:expr) => { ($a) . $rel (& $b) };
   | |_^

error: unused macro definition
  --> library/core/src/tuple.rs:85:1
  --> library/core/src/tuple.rs:85:1
   |
85 | / macro_rules! lexical_partial_cmp {
86 | |     ($a:expr, $b:expr, $($rest_a:expr, $rest_b:expr),+) => {
87 | |         match ($a).partial_cmp(&$b) {
88 | |             Some(Equal) => lexical_partial_cmp!($($rest_a, $rest_b),+),
...  |
92 | |     ($a:expr, $b:expr) => { ($a).partial_cmp(&$b) };
   | |_^

error: unused macro definition
   --> library/core/src/tuple.rs:95:1
   --> library/core/src/tuple.rs:95:1
    |
95  | / macro_rules! lexical_cmp {
96  | |     ($a:expr, $b:expr, $($rest_a:expr, $rest_b:expr),+) => {
97  | |         match ($a).cmp(&$b) {
98  | |             Equal => lexical_cmp!($($rest_a, $rest_b),+),
...   |
102 | |     ($a:expr, $b:expr) => { ($a).cmp(&$b) };
    | |_^


error[E0204]: the trait `Copy` may not be implemented for this type
   --> library/core/src/num/dec2flt/rawfp.rs:30:10
    |
30  |   #[derive(Copy, Clone, Debug)]
    |            ^^^^ in this macro invocation
31  |   pub struct Unpacked {
32  |       pub sig: u64,
    |       ------------ this field does not implement `Copy`
33  |       pub k: i16,
    |       ---------- this field does not implement `Copy`
   ::: library/core/src/marker.rs:393:1
    |
    |
393 | / pub macro Copy($item:item) {
394 | |     /* compiler built-in */
395 | | }
    | |_- in this expansion of `#[derive(Copy)]`

error[E0204]: the trait `Copy` may not be implemented for this type
   --> library/core/src/num/diy_float.rs:13:10
    |
13  |   #[derive(Copy, Clone, Debug)]
    |            ^^^^ in this macro invocation
17  |       pub f: u64,
17  |       pub f: u64,
    |       ---------- this field does not implement `Copy`
18  |       /// The exponent in base 2.
19  |       pub e: i16,
    |       ---------- this field does not implement `Copy`
   ::: library/core/src/marker.rs:393:1
    |
    |
393 | / pub macro Copy($item:item) {
394 | |     /* compiler built-in */
395 | | }
    | |_- in this expansion of `#[derive(Copy)]`

error[E0204]: the trait `Copy` may not be implemented for this type
   --> library/core/src/num/flt2dec/decoder.rs:13:10
    |
13  |   #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    |            ^^^^ in this macro invocation
...
16  |       pub mant: u64,
    |       ------------- this field does not implement `Copy`
17  |       /// The lower error range.
18  |       pub minus: u64,
    |       -------------- this field does not implement `Copy`
19  |       /// The upper error range.
20  |       pub plus: u64,
    |       ------------- this field does not implement `Copy`
21  |       /// The shared exponent in base 2.
22  |       pub exp: i16,
    |       ------------ this field does not implement `Copy`
...
26  |       pub inclusive: bool,
    |       ------------------- this field does not implement `Copy`
   ::: library/core/src/marker.rs:393:1
    |
    |
393 | / pub macro Copy($item:item) {
394 | |     /* compiler built-in */
395 | | }
    | |_- in this expansion of `#[derive(Copy)]`

error[E0204]: the trait `Copy` may not be implemented for this type
   --> library/core/src/num/flt2dec/mod.rs:174:10
    |
174 |   #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    |            ^^^^ in this macro invocation
179 |       Num(u16),
179 |       Num(u16),
    |           --- this field does not implement `Copy`
   ::: library/core/src/marker.rs:393:1
    |
    |
393 | / pub macro Copy($item:item) {
394 | |     /* compiler built-in */
395 | | }
    | |_- in this expansion of `#[derive(Copy)]`

error[E0204]: the trait `Copy` may not be implemented for this type
    |
25  | / macro_rules! nonzero_integers {
25  | / macro_rules! nonzero_integers {
26  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
27  | |         $(
28  | |             /// An integer that is known not to equal zero.
...   |
38  | |             #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    | |                      ^^^^ in this macro invocation (#2)
...   |
42  | |             pub struct $Ty($Int);
    | |                            ---- this field does not implement `Copy`
145 | |     }
146 | | }
146 | | }
    | |_- in this expansion of `nonzero_integers!` (#1)
147 | 
148 | / nonzero_integers! {
149 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
150 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
151 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
...   |
160 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
    | |_- in this macro invocation (#1)
    | 
   ::: library/core/src/marker.rs:393:1
    |
    |
393 | / pub macro Copy($item:item) {
394 | |     /* compiler built-in */
395 | | }
    | |_- in this expansion of `#[derive(Copy)]` (#2)

error[E0204]: the trait `Copy` may not be implemented for this type
   --> library/core/src/ptr/unique.rs:152:17
    |
43  |     _marker: PhantomData<T>,
    |     ----------------------- this field does not implement `Copy`
...
152 | impl<T: ?Sized> Copy for Unique<T> {}


error[E0204]: the trait `Copy` may not be implemented for this type
    |
    |
438 |   #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    |                   ^^^^ in this macro invocation
441 |       t: u64,
441 |       t: u64,
    |       ------ this field does not implement `Copy`
   ::: library/core/src/marker.rs:393:1
    |
    |
393 | / pub macro Copy($item:item) {
394 | |     /* compiler built-in */
395 | | }
    | |_- in this expansion of `#[derive(Copy)]`

error[E0204]: the trait `Copy` may not be implemented for this type
    |
    |
176 |   #[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    |            ^^^^ in this macro invocation
180 |       line: u32,
180 |       line: u32,
    |       --------- this field does not implement `Copy`
181 |       col: u32,
    |       -------- this field does not implement `Copy`
   ::: library/core/src/marker.rs:393:1
    |
    |
393 | / pub macro Copy($item:item) {
394 | |     /* compiler built-in */
395 | | }
    | |_- in this expansion of `#[derive(Copy)]`

error[E0204]: the trait `Copy` may not be implemented for this type
   --> library/core/src/fmt/rt/v1.rs:14:10
    |
14  |   #[derive(Copy, Clone)]
    |            ^^^^ in this macro invocation
15  |   pub struct FormatSpec {
16  |       pub fill: char,
    |       -------------- this field does not implement `Copy`
17  |       pub align: Alignment,
18  |       pub flags: u32,
    |       -------------- this field does not implement `Copy`
   ::: library/core/src/marker.rs:393:1
    |
    |
393 | / pub macro Copy($item:item) {
394 | |     /* compiler built-in */
395 | | }
    | |_- in this expansion of `#[derive(Copy)]`

error[E0204]: the trait `Copy` may not be implemented for this type
   --> library/core/src/hash/sip.rs:71:24
    |
71  |   #[derive(Debug, Clone, Copy)]
    |                          ^^^^ in this macro invocation
78  |       v0: u64,
78  |       v0: u64,
    |       ------- this field does not implement `Copy`
79  |       v2: u64,
    |       ------- this field does not implement `Copy`
80  |       v1: u64,
    |       ------- this field does not implement `Copy`
81  |       v3: u64,
    |       ------- this field does not implement `Copy`
   ::: library/core/src/marker.rs:393:1
    |
    |
393 | / pub macro Copy($item:item) {
394 | |     /* compiler built-in */
395 | | }
    | |_- in this expansion of `#[derive(Copy)]`

error[E0204]: the trait `Copy` may not be implemented for this type
     |
     |
1971 |   #[derive(Debug, Clone, Copy)]
     |                          ^^^^ in this macro invocation
...
1976 |       marker: PhantomData<&'a [T; N]>,
     |       ------------------------------- this field does not implement `Copy`
    ::: library/core/src/marker.rs:393:1
     |
     |
393  | / pub macro Copy($item:item) {
394  | |     /* compiler built-in */
395  | | }
     | |_- in this expansion of `#[derive(Copy)]`

error[E0204]: the trait `Copy` may not be implemented for this type
    |
    |
44  |   #[derive(Copy, Eq, PartialEq, Clone, Debug)]
    |            ^^^^ in this macro invocation
...
48  |       pub(super) error_len: Option<u8>,
    |       -------------------------------- this field does not implement `Copy`
   ::: library/core/src/marker.rs:393:1
    |
    |
393 | / pub macro Copy($item:item) {
394 | |     /* compiler built-in */
395 | | }
    | |_- in this expansion of `#[derive(Copy)]`

error[E0204]: the trait `Copy` may not be implemented for this type
    |
    |
52  |   #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    |                   ^^^^ in this macro invocation
53  |   pub struct Duration {
54  |       secs: u64,
    |       --------- this field does not implement `Copy`
55  |       nanos: u32, // Always 0 <= nanos < NANOS_PER_SEC
    |       ---------- this field does not implement `Copy`
   ::: library/core/src/marker.rs:393:1
    |
    |
393 | / pub macro Copy($item:item) {
394 | |     /* compiler built-in */
395 | | }
    | |_- in this expansion of `#[derive(Copy)]`

error[E0204]: the trait `Copy` may not be implemented for this type
    |
    |
5   | / macro_rules! simd_ty {
6   | |     ($id:ident [$ety:ident]: $($elem_ty:ident),* | $($elem_name:ident),*) => {
7   | |         #[repr(simd)]
8   | |         #[derive(Copy, Clone, Debug, PartialEq)]
    | |                  ^^^^ in this macro invocation (#2)
9   | |         pub(crate) struct $id($(pub $elem_ty),*);
    | |                                 |
    | |                                 |
    | |                                 this field does not implement `Copy`
    | |                                 this field does not implement `Copy`
35  | |     }
36  | | }
36  | | }
    | |_- in this expansion of `simd_ty!` (#1)
...
80  |   simd_ty!(u8x2[u8]: u8, u8 | x0, x1);
    |   ------------------------------------ in this macro invocation (#1)
   ::: library/core/src/marker.rs:393:1
    |
    |
393 | / pub macro Copy($item:item) {
394 | |     /* compiler built-in */
395 | | }
    | |_- in this expansion of `#[derive(Copy)]` (#2)

error[E0204]: the trait `Copy` may not be implemented for this type
    |
    |
5   | / macro_rules! simd_ty {
6   | |     ($id:ident [$ety:ident]: $($elem_ty:ident),* | $($elem_name:ident),*) => {
7   | |         #[repr(simd)]
8   | |         #[derive(Copy, Clone, Debug, PartialEq)]
    | |                  ^^^^ in this macro invocation (#2)
9   | |         pub(crate) struct $id($(pub $elem_ty),*);
    | |                                 |
    | |                                 |
    | |                                 this field does not implement `Copy`
    | |                                 this field does not implement `Copy`
35  | |     }
36  | | }
36  | | }
    | |_- in this expansion of `simd_ty!` (#1)
...
81  |   simd_ty!(i8x2[i8]: i8, i8 | x0, x1);
    |   ------------------------------------ in this macro invocation (#1)
   ::: library/core/src/marker.rs:393:1
    |
    |
393 | / pub macro Copy($item:item) {
394 | |     /* compiler built-in */
395 | | }
    | |_- in this expansion of `#[derive(Copy)]` (#2)

error[E0204]: the trait `Copy` may not be implemented for this type
    |
    |
5   | / macro_rules! simd_ty {
6   | |     ($id:ident [$ety:ident]: $($elem_ty:ident),* | $($elem_name:ident),*) => {
7   | |         #[repr(simd)]
8   | |         #[derive(Copy, Clone, Debug, PartialEq)]
    | |                  ^^^^ in this macro invocation (#2)
9   | |         pub(crate) struct $id($(pub $elem_ty),*);
    | |                                 |
    | |                                 |
    | |                                 this field does not implement `Copy`
    | |                                 this field does not implement `Copy`
    | |                                 this field does not implement `Copy`
    | |                                 this field does not implement `Copy`
35  | |     }
36  | | }
36  | | }
    | |_- in this expansion of `simd_ty!` (#1)
...
85  |   simd_ty!(u8x4[u8]: u8, u8, u8, u8 | x0, x1, x2, x3);
    |   ---------------------------------------------------- in this macro invocation (#1)
   ::: library/core/src/marker.rs:393:1
    |
    |
393 | / pub macro Copy($item:item) {
394 | |     /* compiler built-in */
395 | | }
    | |_- in this expansion of `#[derive(Copy)]` (#2)

error[E0204]: the trait `Copy` may not be implemented for this type
    |
    |
5   | / macro_rules! simd_ty {
6   | |     ($id:ident [$ety:ident]: $($elem_ty:ident),* | $($elem_name:ident),*) => {
7   | |         #[repr(simd)]
8   | |         #[derive(Copy, Clone, Debug, PartialEq)]
    | |                  ^^^^ in this macro invocation (#2)
9   | |         pub(crate) struct $id($(pub $elem_ty),*);
    | |                                 |
    | |                                 |
    | |                                 this field does not implement `Copy`
    | |                                 this field does not implement `Copy`
35  | |     }
36  | | }
36  | | }
    | |_- in this expansion of `simd_ty!` (#1)
...
86  |   simd_ty!(u16x2[u16]: u16, u16 | x0, x1);
    |   ---------------------------------------- in this macro invocation (#1)
   ::: library/core/src/marker.rs:393:1
    |
    |
393 | / pub macro Copy($item:item) {
394 | |     /* compiler built-in */
395 | | }
    | |_- in this expansion of `#[derive(Copy)]` (#2)

error[E0204]: the trait `Copy` may not be implemented for this type
    |
    |
5   | / macro_rules! simd_ty {
6   | |     ($id:ident [$ety:ident]: $($elem_ty:ident),* | $($elem_name:ident),*) => {
7   | |         #[repr(simd)]
8   | |         #[derive(Copy, Clone, Debug, PartialEq)]
    | |                  ^^^^ in this macro invocation (#2)
9   | |         pub(crate) struct $id($(pub $elem_ty),*);
    | |                                 |
    | |                                 |
    | |                                 this field does not implement `Copy`
    | |                                 this field does not implement `Copy`
    | |                                 this field does not implement `Copy`
    | |                                 this field does not implement `Copy`
35  | |     }
36  | | }
36  | | }
    | |_- in this expansion of `simd_ty!` (#1)
...
88  |   simd_ty!(i8x4[i8]: i8, i8, i8, i8 | x0, x1, x2, x3);
    |   ---------------------------------------------------- in this macro invocation (#1)
   ::: library/core/src/marker.rs:393:1
    |
    |
393 | / pub macro Copy($item:item) {
