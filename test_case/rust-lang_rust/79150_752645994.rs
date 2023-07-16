plain
   Compiling std v0.0.0 (/checkout/library/std)
error: unexpected token: `concat`
  --> library/core/src/num/shells/int_macros.rs:6:17
   |
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
43 | |     )
44 | | }
   | |_- in this expansion of `int_module!`
   | |_- in this expansion of `int_module!`
   | 
  ::: library/core/src/num/shells/i128.rs:10:1
   |
10 |   int_module! { i128, #[stable(feature = "i128", since="1.26.0")] }
   |   ----------------------------------------------------------------- in this macro invocation
error: unexpected token: `concat`
  --> library/core/src/num/shells/int_macros.rs:6:17
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
43 | |     )
44 | | }
   | | -
   | | -
   | | |
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i16.rs:10:1
   |
   |
10 |   int_module! { i16 }
   |   ------------------- in this macro invocation (#1)
error: unexpected token: `concat`
  --> library/core/src/num/shells/int_macros.rs:6:17
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
43 | |     )
44 | | }
   | | -
   | | -
   | | |
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i32.rs:10:1
   |
   |
10 |   int_module! { i32 }
   |   ------------------- in this macro invocation (#1)
error: unexpected token: `concat`
  --> library/core/src/num/shells/int_macros.rs:6:17
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
43 | |     )
44 | | }
   | | -
   | | -
   | | |
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i64.rs:10:1
   |
   |
10 |   int_module! { i64 }
   |   ------------------- in this macro invocation (#1)
error: unexpected token: `concat`
  --> library/core/src/num/shells/int_macros.rs:6:17
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
43 | |     )
44 | | }
   | | -
   | | -
   | | |
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i8.rs:10:1
   |
   |
10 |   int_module! { i8 }
   |   ------------------ in this macro invocation (#1)
error: unexpected token: `concat`
  --> library/core/src/num/shells/int_macros.rs:6:17
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
43 | |     )
44 | | }
   | | -
   | | -
   | | |
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/isize.rs:10:1
   |
   |
10 |   int_module! { isize }
   |   --------------------- in this macro invocation (#1)
error: unexpected token: `concat`
  --> library/core/src/num/shells/int_macros.rs:6:17
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
43 | |     )
44 | | }
   | |_- in this expansion of `int_module!`
   | |_- in this expansion of `int_module!`
   | 
  ::: library/core/src/num/shells/u128.rs:9:1
   |
9  |   int_module! { u128, #[stable(feature = "i128", since="1.26.0")] }
   |   ----------------------------------------------------------------- in this macro invocation
error: unexpected token: `concat`
  --> library/core/src/num/shells/int_macros.rs:6:17
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
43 | |     )
44 | | }
   | | -
   | | -
   | | |
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/u16.rs:10:1
   |
   |
10 |   int_module! { u16 }
   |   ------------------- in this macro invocation (#1)
error: unexpected token: `concat`
  --> library/core/src/num/shells/int_macros.rs:6:17
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
43 | |     )
44 | | }
   | | -
   | | -
   | | |
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/u32.rs:10:1
   |
   |
10 |   int_module! { u32 }
   |   ------------------- in this macro invocation (#1)
error: unexpected token: `concat`
  --> library/core/src/num/shells/int_macros.rs:6:17
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
43 | |     )
44 | | }
   | | -
   | | -
   | | |
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/u64.rs:10:1
   |
   |
10 |   int_module! { u64 }
   |   ------------------- in this macro invocation (#1)
error: unexpected token: `concat`
  --> library/core/src/num/shells/int_macros.rs:6:17
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
43 | |     )
44 | | }
   | | -
   | | -
   | | |
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/u8.rs:10:1
   |
   |
10 |   int_module! { u8 }
   |   ------------------ in this macro invocation (#1)
error: unexpected token: `concat`
  --> library/core/src/num/shells/int_macros.rs:6:17
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
43 | |     )
44 | | }
   | | -
   | | -
   | | |
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/usize.rs:10:1
   |
   |
10 |   int_module! { usize }
   |   --------------------- in this macro invocation (#1)
error: unexpected token: `concat`
   --> library/core/src/num/nonzero.rs:31:21
    |
25  | / macro_rules! nonzero_integers {
25  | / macro_rules! nonzero_integers {
26  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
27  | |         $(
28  | |             /// An integer that is known not to equal zero.
...   |
31  | |             #[doc = concat!("For example, `Option<", stringify!($Ty), ">` is the same size as `", stringify!($Int), "`:")]
...   |
145 | |     }
146 | | }
146 | | }
    | |_- in this expansion of `nonzero_integers!`
147 | 
148 | / nonzero_integers! {
149 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
150 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
151 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
...   |
160 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
    | |_- in this macro invocation

error: unexpected token: `concat`
   --> library/core/src/num/wrapping.rs:416:21
   --> library/core/src/num/wrapping.rs:416:21
    |
403 | / macro_rules! wrapping_int_impl {
404 | |     ($($t:ty)*) => ($(
405 | |         impl Wrapping<$t> {
    | |                           - while parsing this item list starting here
406 | |             /// Returns the smallest value that can be represented by this integer type.
...   |
416 | |             #[doc = concat!("assert_eq!(<Wrapping<", stringify!($t), ">>::MIN, Wrapping(", stringify!($t), "::MIN));")]
...   |
735 | |         }
735 | |         }
    | |         - the item list ends here
736 | |     )*)
737 | | }
    | |_- in this expansion of `wrapping_int_impl!`
738 | 
739 |   wrapping_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
    |   -------------------------------------------------------------------------- in this macro invocation
error: unexpected token: `concat`
   --> library/core/src/num/wrapping.rs:754:21
    |
    |
741 | / macro_rules! wrapping_int_impl_signed {
742 | |     ($($t:ty)*) => ($(
743 | |         impl Wrapping<$t> {
    | |                           - while parsing this item list starting here
744 | |             /// Returns the number of leading zeros in the binary representation of `self`.
...   |
754 | |             #[doc = concat!("let n = Wrapping(", stringify!($t), "::MAX) >> 2;")]
...   |
853 | |         }
853 | |         }
    | |         - the item list ends here
854 | |     )*)
855 | | }
    | |_- in this expansion of `wrapping_int_impl_signed!`
856 | 
857 |   wrapping_int_impl_signed! { isize i8 i16 i32 i64 i128 }
    |   ------------------------------------------------------- in this macro invocation
error: unexpected token: `concat`
   --> library/core/src/num/wrapping.rs:872:21
    |
    |
859 | / macro_rules! wrapping_int_impl_unsigned {
860 | |     ($($t:ty)*) => ($(
861 | |         impl Wrapping<$t> {
    | |                           - while parsing this item list starting here
862 | |             /// Returns the number of leading zeros in the binary representation of `self`.
...   |
872 | |             #[doc = concat!("let n = Wrapping(", stringify!($t), "::MAX) >> 2;")]
...   |
924 | |         }
924 | |         }
    | |         - the item list ends here
925 | |     )*)
926 | | }
    | |_- in this expansion of `wrapping_int_impl_unsigned!`
927 | 
928 |   wrapping_int_impl_unsigned! { usize u8 u16 u32 u64 u128 }
    |   --------------------------------------------------------- in this macro invocation
error: unexpected token: `concat`
    --> library/core/src/num/int_macros.rs:13:17
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
13   |  |         #[doc = concat!("assert_eq!(", stringify!($SelfT), "::MIN, ", stringify!($Min), ");")]
     |  |                 ^^^^^^
2007 |  |     }
2008 |  | }
2008 |  | }
     |  |_- in this expansion of `int_impl!`
    ::: library/core/src/num/mod.rs:91:5
     |
     |
91   | /      int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
92   | |      "[0x12]", "[0x12]", "", "" }
     | |_________________________________- in this macro invocation
error: unexpected token: `concat`
    --> library/core/src/num/int_macros.rs:13:17
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
13   |  |         #[doc = concat!("assert_eq!(", stringify!($SelfT), "::MIN, ", stringify!($Min), ");")]
     |  |                 ^^^^^^
2007 |  |     }
2008 |  | }
2008 |  | }
     |  |_- in this expansion of `int_impl!`
    ::: library/core/src/num/mod.rs:97:5
     |
     |
97   | /      int_impl! { i16, i16, u16, 16, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
98   | |      "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     | |_______________________________________________________- in this macro invocation
error: unexpected token: `concat`
    --> library/core/src/num/int_macros.rs:13:17
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
13   |  |         #[doc = concat!("assert_eq!(", stringify!($SelfT), "::MIN, ", stringify!($Min), ");")]
     |  |                 ^^^^^^
2007 |  |     }
2008 |  | }
2008 |  | }
     |  |_- in this expansion of `int_impl!`
    ::: library/core/src/num/mod.rs:103:5
     |
     |
103  | /      int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
104  | |      "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
105  | |      "[0x12, 0x34, 0x56, 0x78]", "", "" }
     | |_________________________________________- in this macro invocation
error: unexpected token: `concat`
    --> library/core/src/num/int_macros.rs:13:17
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
13   |  |         #[doc = concat!("assert_eq!(", stringify!($SelfT), "::MIN, ", stringify!($Min), ");")]
     |  |                 ^^^^^^
2007 |  |     }
2008 |  | }
2008 |  | }
     |  |_- in this expansion of `int_impl!`
    ::: library/core/src/num/mod.rs:110:5
     |
     |
110  | /      int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, 12,
111  | |      "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
112  | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
113  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
     | |_________________________________________________________________- in this macro invocation
error: unexpected token: `concat`
    --> library/core/src/num/int_macros.rs:13:17
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
13   |  |         #[doc = concat!("assert_eq!(", stringify!($SelfT), "::MIN, ", stringify!($Min), ");")]
     |  |                 ^^^^^^
2007 |  |     }
2008 |  | }
2008 |  | }
     |  |_- in this expansion of `int_impl!`
    ::: library/core/src/num/mod.rs:118:5
     |
     |
118  | /      int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
119  | |      170141183460469231731687303715884105727, 16,
120  | |      "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
121  | |      "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
124  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
125  | |        0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
     | |_________________________________________________________________- in this macro invocation
error: unexpected token: `concat`
    --> library/core/src/num/int_macros.rs:13:17
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
13   |  |         #[doc = concat!("assert_eq!(", stringify!($SelfT), "::MIN, ", stringify!($Min), ");")]
     |  |                 ^^^^^^
2007 |  |     }
2008 |  | }
2008 |  | }
     |  |_- in this expansion of `int_impl!`
    ::: library/core/src/num/mod.rs:148:5
     |
     |
148  | /      int_impl! { isize, i64, usize, 64, -9223372036854775808, 9223372036854775807,
149  | |      12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
150  | |       "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
151  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
152  | |       usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
     | |________________________________________________________________________- in this macro invocation
error: unexpected token: `concat`
    --> library/core/src/num/uint_macros.rs:13:17
     |
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
13   |  |         #[doc = concat!("assert_eq!(", stringify!($SelfT), "::MIN, 0);")]
     |  |                 ^^^^^^
1833 |  |     }
1834 |  | }
1834 |  | }
     |  |_- in this expansion of `uint_impl!`
    ::: library/core/src/num/mod.rs:157:5
     |
     |
157  | /      uint_impl! { u8, u8, 8, 255, 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
158  | |      "[0x12]", "", "" }
---
  |
5 | use crate::intrinsics;
  |     ^^^^^^^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `crate::mem`
 --> library/core/src/num/mod.rs:6:5
  |
6 | use crate::mem;
6 | use crate::mem;
  |     ^^^^^^^^^^

error: unused macro definition
  --> library/core/src/num/mod.rs:10:1
   |
10 | / macro_rules! try_opt {
11 | |     ($e:expr) => {
12 | |         match $e {
13 | |             Some(x) => x,
16 | |     };
17 | | }
   | |_^
   |
   |
   = note: `-D unused-macros` implied by `-D warnings`
error: unused macro definition
  --> library/core/src/num/mod.rs:20:1
   |
   |
20 | / macro_rules! unlikely {
21 | |     ($e: expr) => {
22 | |         intrinsics::unlikely($e)
24 | | }
   | |_^

error: unused import: `crate::fmt`
error: unused import: `crate::fmt`
 --> library/core/src/num/nonzero.rs:3:5
  |
3 | use crate::fmt;
  |     ^^^^^^^^^^

error: unused imports: `BitOrAssign`, `BitOr`
  |
  |
4 | use crate::ops::{BitOr, BitOrAssign, Div, Rem};
  |                  ^^^^^  ^^^^^^^^^^^
error: unused import: `crate::intrinsics`
 --> library/core/src/num/nonzero.rs:9:5
  |
9 | use crate::intrinsics;
9 | use crate::intrinsics;
  |     ^^^^^^^^^^^^^^^^^

error: unused macro definition
  --> library/core/src/num/nonzero.rs:11:1
   |
11 | / macro_rules! impl_nonzero_fmt {
12 | |     ( #[$stability: meta] ( $( $Trait: ident ),+ ) for $Ty: ident ) => {
13 | |         $(
14 | |             #[$stability]
22 | |     }
23 | | }
   | |_^


error: unused macro definition
  --> library/core/src/num/mod.rs:67:1
   |
67 | / macro_rules! usize_isize_to_xe_bytes_doc {
68 | |     () => {
70 | |
...  |
75 | |     };
76 | | }
76 | | }
   | |_^

error: unused macro definition
  --> library/core/src/num/mod.rs:78:1
   |
78 | / macro_rules! usize_isize_from_xe_bytes_doc {
79 | |     () => {
81 | |
...  |
86 | |     };
87 | | }
87 | | }
   | |_^

error[E0599]: no function or associated item named `new` found for struct `AtomicI8` in the current scope
     |
1372 | / macro_rules! atomic_int {
1372 | / macro_rules! atomic_int {
1373 | |     ($cfg_cas:meta,
1374 | |      $cfg_align:meta,
1375 | |      $stable:meta,
...    |
1410 | |         pub struct $atomic_type {
     | |         ----------------------- function or associated item `new` not found for this
...    |
1421 | |         pub const $atomic_init: $atomic_type = $atomic_type::new(0);
     | |                                                              ^^^ function or associated item not found in `AtomicI8`
2149 | |     }
2150 | | }
     | |_- in this expansion of `atomic_int!`
...
...
2153 | / atomic_int! {
2154 | |     cfg(target_has_atomic = "8"),
2155 | |     cfg(target_has_atomic_equal_alignment = "8"),
2156 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2169 | |     i8 AtomicI8 ATOMIC_I8_INIT
     | |_- in this macro invocation
     |
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `ZipImpl` defines an item `new`, perhaps you need to implement it
    --> library/core/src/iter/adapters/zip.rs:83:1
     |
83   | trait ZipImpl<A, B> {

error[E0046]: not all trait items implemented, missing: `from`
    --> library/core/src/sync/atomic.rs:1432:9
     |
     |
1372 | / macro_rules! atomic_int {
1373 | |     ($cfg_cas:meta,
1374 | |      $cfg_align:meta,
1375 | |      $stable:meta,
...    |
1432 | |         impl From<$int_type> for $atomic_type {
     | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `from` in implementation
2149 | |     }
2150 | | }
     | |_- in this expansion of `atomic_int!`
...
...
2153 | / atomic_int! {
2154 | |     cfg(target_has_atomic = "8"),
2155 | |     cfg(target_has_atomic_equal_alignment = "8"),
2156 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2169 | |     i8 AtomicI8 ATOMIC_I8_INIT
     | |_- in this macro invocation
     | 
    ::: library/core/src/convert/mod.rs:377:5
     |
     |
377  |       fn from(_: T) -> Self;
     |       ---------------------- `from` from trait
error[E0599]: no function or associated item named `new` found for struct `AtomicU8` in the current scope
    --> library/core/src/sync/atomic.rs:1421:62
     |
1372 | / macro_rules! atomic_int {
1372 | / macro_rules! atomic_int {
1373 | |     ($cfg_cas:meta,
1374 | |      $cfg_align:meta,
1375 | |      $stable:meta,
...    |
1410 | |         pub struct $atomic_type {
     | |         ----------------------- function or associated item `new` not found for this
...    |
1421 | |         pub const $atomic_init: $atomic_type = $atomic_type::new(0);
     | |                                                              ^^^ function or associated item not found in `AtomicU8`
2149 | |     }
2150 | | }
     | |_- in this expansion of `atomic_int!`
...
...
2172 | / atomic_int! {
2173 | |     cfg(target_has_atomic = "8"),
2174 | |     cfg(target_has_atomic_equal_alignment = "8"),
2175 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2188 | |     u8 AtomicU8 ATOMIC_U8_INIT
     | |_- in this macro invocation
     |
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `ZipImpl` defines an item `new`, perhaps you need to implement it
    --> library/core/src/iter/adapters/zip.rs:83:1
     |
83   | trait ZipImpl<A, B> {

error[E0046]: not all trait items implemented, missing: `from`
    --> library/core/src/sync/atomic.rs:1432:9
     |
     |
1372 | / macro_rules! atomic_int {
1373 | |     ($cfg_cas:meta,
1374 | |      $cfg_align:meta,
1375 | |      $stable:meta,
...    |
1432 | |         impl From<$int_type> for $atomic_type {
     | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `from` in implementation
2149 | |     }
2150 | | }
     | |_- in this expansion of `atomic_int!`
...
...
2172 | / atomic_int! {
2173 | |     cfg(target_has_atomic = "8"),
2174 | |     cfg(target_has_atomic_equal_alignment = "8"),
2175 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2188 | |     u8 AtomicU8 ATOMIC_U8_INIT
     | |_- in this macro invocation
     | 
    ::: library/core/src/convert/mod.rs:377:5
     |
     |
377  |       fn from(_: T) -> Self;
     |       ---------------------- `from` from trait

error[E0599]: no function or associated item named `new` found for struct `AtomicI16` in the current scope
     |
1372 | / macro_rules! atomic_int {
1372 | / macro_rules! atomic_int {
1373 | |     ($cfg_cas:meta,
1374 | |      $cfg_align:meta,
1375 | |      $stable:meta,
...    |
1410 | |         pub struct $atomic_type {
     | |         ----------------------- function or associated item `new` not found for this
...    |
1421 | |         pub const $atomic_init: $atomic_type = $atomic_type::new(0);
     | |                                                              ^^^ function or associated item not found in `AtomicI16`
2149 | |     }
2150 | | }
     | |_- in this expansion of `atomic_int!`
...
...
2191 | / atomic_int! {
2192 | |     cfg(target_has_atomic = "16"),
2193 | |     cfg(target_has_atomic_equal_alignment = "16"),
2194 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2207 | |     i16 AtomicI16 ATOMIC_I16_INIT
     | |_- in this macro invocation
     |
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `ZipImpl` defines an item `new`, perhaps you need to implement it
    --> library/core/src/iter/adapters/zip.rs:83:1
     |
83   | trait ZipImpl<A, B> {

error[E0046]: not all trait items implemented, missing: `from`
    --> library/core/src/sync/atomic.rs:1432:9
     |
     |
1372 | / macro_rules! atomic_int {
1373 | |     ($cfg_cas:meta,
1374 | |      $cfg_align:meta,
1375 | |      $stable:meta,
...    |
1432 | |         impl From<$int_type> for $atomic_type {
     | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `from` in implementation
2149 | |     }
2150 | | }
     | |_- in this expansion of `atomic_int!`
...
...
2191 | / atomic_int! {
2192 | |     cfg(target_has_atomic = "16"),
2193 | |     cfg(target_has_atomic_equal_alignment = "16"),
2194 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2207 | |     i16 AtomicI16 ATOMIC_I16_INIT
     | |_- in this macro invocation
     | 
    ::: library/core/src/convert/mod.rs:377:5
     |
     |
377  |       fn from(_: T) -> Self;
     |       ---------------------- `from` from trait
error[E0599]: no function or associated item named `new` found for struct `AtomicU16` in the current scope
    --> library/core/src/sync/atomic.rs:1421:62
     |
1372 | / macro_rules! atomic_int {
1372 | / macro_rules! atomic_int {
1373 | |     ($cfg_cas:meta,
1374 | |      $cfg_align:meta,
1375 | |      $stable:meta,
...    |
1410 | |         pub struct $atomic_type {
     | |         ----------------------- function or associated item `new` not found for this
...    |
1421 | |         pub const $atomic_init: $atomic_type = $atomic_type::new(0);
     | |                                                              ^^^ function or associated item not found in `AtomicU16`
2149 | |     }
2150 | | }
     | |_- in this expansion of `atomic_int!`
...
...
2210 | / atomic_int! {
2211 | |     cfg(target_has_atomic = "16"),
2212 | |     cfg(target_has_atomic_equal_alignment = "16"),
2213 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2226 | |     u16 AtomicU16 ATOMIC_U16_INIT
     | |_- in this macro invocation
     |
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `ZipImpl` defines an item `new`, perhaps you need to implement it
    --> library/core/src/iter/adapters/zip.rs:83:1
     |
83   | trait ZipImpl<A, B> {

error[E0046]: not all trait items implemented, missing: `from`
    --> library/core/src/sync/atomic.rs:1432:9
     |
     |
1372 | / macro_rules! atomic_int {
1373 | |     ($cfg_cas:meta,
1374 | |      $cfg_align:meta,
1375 | |      $stable:meta,
...    |
1432 | |         impl From<$int_type> for $atomic_type {
     | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `from` in implementation
2149 | |     }
2150 | | }
     | |_- in this expansion of `atomic_int!`
...
...
2210 | / atomic_int! {
2211 | |     cfg(target_has_atomic = "16"),
2212 | |     cfg(target_has_atomic_equal_alignment = "16"),
2213 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2226 | |     u16 AtomicU16 ATOMIC_U16_INIT
     | |_- in this macro invocation
     | 
    ::: library/core/src/convert/mod.rs:377:5
     |
     |
377  |       fn from(_: T) -> Self;
     |       ---------------------- `from` from trait

error[E0599]: no function or associated item named `new` found for struct `AtomicI32` in the current scope
     |
1372 | / macro_rules! atomic_int {
1372 | / macro_rules! atomic_int {
1373 | |     ($cfg_cas:meta,
1374 | |      $cfg_align:meta,
1375 | |      $stable:meta,
...    |
1410 | |         pub struct $atomic_type {
     | |         ----------------------- function or associated item `new` not found for this
...    |
1421 | |         pub const $atomic_init: $atomic_type = $atomic_type::new(0);
     | |                                                              ^^^ function or associated item not found in `AtomicI32`
2149 | |     }
2150 | | }
     | |_- in this expansion of `atomic_int!`
...
...
2229 | / atomic_int! {
2230 | |     cfg(target_has_atomic = "32"),
2231 | |     cfg(target_has_atomic_equal_alignment = "32"),
2232 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2245 | |     i32 AtomicI32 ATOMIC_I32_INIT
     | |_- in this macro invocation
     |
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `ZipImpl` defines an item `new`, perhaps you need to implement it
    --> library/core/src/iter/adapters/zip.rs:83:1
     |
83   | trait ZipImpl<A, B> {

error[E0046]: not all trait items implemented, missing: `from`
    --> library/core/src/sync/atomic.rs:1432:9
     |
     |
1372 | / macro_rules! atomic_int {
1373 | |     ($cfg_cas:meta,
1374 | |      $cfg_align:meta,
1375 | |      $stable:meta,
...    |
1432 | |         impl From<$int_type> for $atomic_type {
     | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `from` in implementation
2149 | |     }
2150 | | }
     | |_- in this expansion of `atomic_int!`
...
...
2229 | / atomic_int! {
2230 | |     cfg(target_has_atomic = "32"),
2231 | |     cfg(target_has_atomic_equal_alignment = "32"),
2232 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2245 | |     i32 AtomicI32 ATOMIC_I32_INIT
     | |_- in this macro invocation
     | 
    ::: library/core/src/convert/mod.rs:377:5
     |
     |
377  |       fn from(_: T) -> Self;
     |       ---------------------- `from` from trait
error[E0599]: no function or associated item named `new` found for struct `AtomicU32` in the current scope
    --> library/core/src/sync/atomic.rs:1421:62
     |
1372 | / macro_rules! atomic_int {
1372 | / macro_rules! atomic_int {
1373 | |     ($cfg_cas:meta,
1374 | |      $cfg_align:meta,
1375 | |      $stable:meta,
...    |
1410 | |         pub struct $atomic_type {
     | |         ----------------------- function or associated item `new` not found for this
...    |
1421 | |         pub const $atomic_init: $atomic_type = $atomic_type::new(0);
     | |                                                              ^^^ function or associated item not found in `AtomicU32`
2149 | |     }
2150 | | }
     | |_- in this expansion of `atomic_int!`
...
...
2248 | / atomic_int! {
2249 | |     cfg(target_has_atomic = "32"),
2250 | |     cfg(target_has_atomic_equal_alignment = "32"),
2251 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2264 | |     u32 AtomicU32 ATOMIC_U32_INIT
     | |_- in this macro invocation
     |
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `ZipImpl` defines an item `new`, perhaps you need to implement it
    --> library/core/src/iter/adapters/zip.rs:83:1
     |
83   | trait ZipImpl<A, B> {

error[E0046]: not all trait items implemented, missing: `from`
    --> library/core/src/sync/atomic.rs:1432:9
     |
     |
1372 | / macro_rules! atomic_int {
1373 | |     ($cfg_cas:meta,
1374 | |      $cfg_align:meta,
1375 | |      $stable:meta,
...    |
1432 | |         impl From<$int_type> for $atomic_type {
     | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `from` in implementation
2149 | |     }
2150 | | }
     | |_- in this expansion of `atomic_int!`
...
...
2248 | / atomic_int! {
2249 | |     cfg(target_has_atomic = "32"),
2250 | |     cfg(target_has_atomic_equal_alignment = "32"),
2251 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2264 | |     u32 AtomicU32 ATOMIC_U32_INIT
     | |_- in this macro invocation
     | 
    ::: library/core/src/convert/mod.rs:377:5
     |
     |
377  |       fn from(_: T) -> Self;
     |       ---------------------- `from` from trait

error[E0599]: no function or associated item named `new` found for struct `AtomicI64` in the current scope
     |
1372 | / macro_rules! atomic_int {
1372 | / macro_rules! atomic_int {
1373 | |     ($cfg_cas:meta,
1374 | |      $cfg_align:meta,
1375 | |      $stable:meta,
...    |
1410 | |         pub struct $atomic_type {
     | |         ----------------------- function or associated item `new` not found for this
...    |
1421 | |         pub const $atomic_init: $atomic_type = $atomic_type::new(0);
     | |                                                              ^^^ function or associated item not found in `AtomicI64`
2149 | |     }
2150 | | }
     | |_- in this expansion of `atomic_int!`
...
...
2267 | / atomic_int! {
2268 | |     cfg(target_has_atomic = "64"),
2269 | |     cfg(target_has_atomic_equal_alignment = "64"),
2270 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2283 | |     i64 AtomicI64 ATOMIC_I64_INIT
     | |_- in this macro invocation
     |
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `ZipImpl` defines an item `new`, perhaps you need to implement it
    --> library/core/src/iter/adapters/zip.rs:83:1
     |
83   | trait ZipImpl<A, B> {
