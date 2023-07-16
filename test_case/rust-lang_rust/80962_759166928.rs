plain
   Compiling libc v0.2.79
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: `intrinsics::unchecked_div` is not yet stable as a const fn
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
525  |  |                 Some(unsafe { intrinsics::unchecked_div(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
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
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_div` is not yet stable as a const fn
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
525  |  |                 Some(unsafe { intrinsics::unchecked_div(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
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
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_div` is not yet stable as a const fn
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
525  |  |                 Some(unsafe { intrinsics::unchecked_div(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
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
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_div` is not yet stable as a const fn
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
525  |  |                 Some(unsafe { intrinsics::unchecked_div(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
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
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_div` is not yet stable as a const fn
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
525  |  |                 Some(unsafe { intrinsics::unchecked_div(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
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
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_div` is not yet stable as a const fn
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
525  |  |                 Some(unsafe { intrinsics::unchecked_div(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
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
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_rem` is not yet stable as a const fn
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
577  |  |                 Some(unsafe { intrinsics::unchecked_rem(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
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
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_rem` is not yet stable as a const fn
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
577  |  |                 Some(unsafe { intrinsics::unchecked_rem(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
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
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_rem` is not yet stable as a const fn
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
577  |  |                 Some(unsafe { intrinsics::unchecked_rem(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
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
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_rem` is not yet stable as a const fn
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
577  |  |                 Some(unsafe { intrinsics::unchecked_rem(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
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
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_rem` is not yet stable as a const fn
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
577  |  |                 Some(unsafe { intrinsics::unchecked_rem(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
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
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_rem` is not yet stable as a const fn
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
577  |  |                 Some(unsafe { intrinsics::unchecked_rem(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
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
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_div` is not yet stable as a const fn
    --> library/core/src/num/uint_macros.rs:535:31
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
535  |  |                 Some(unsafe { intrinsics::unchecked_div(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
1833 |  |     }
1834 |  | }
1834 |  | }
     |  |_- in this expansion of `uint_impl!`
    ::: library/core/src/num/mod.rs:157:5
     |
     |
157  | /      uint_impl! { u8, u8, 8, 255, 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
158  | |      "[0x12]", "", "" }
     | |_______________________- in this macro invocation
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_div` is not yet stable as a const fn
    --> library/core/src/num/uint_macros.rs:535:31
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
535  |  |                 Some(unsafe { intrinsics::unchecked_div(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
1833 |  |     }
1834 |  | }
1834 |  | }
     |  |_- in this expansion of `uint_impl!`
    ::: library/core/src/num/mod.rs:656:5
     |
     |
656  | /      uint_impl! { u16, u16, 16, 65535, 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
657  | |      "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     | |_____________________________________________- in this macro invocation
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_div` is not yet stable as a const fn
    --> library/core/src/num/uint_macros.rs:535:31
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
535  |  |                 Some(unsafe { intrinsics::unchecked_div(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
1833 |  |     }
1834 |  | }
1834 |  | }
     |  |_- in this expansion of `uint_impl!`
    ::: library/core/src/num/mod.rs:662:5
     |
     |
662  | /      uint_impl! { u32, u32, 32, 4294967295, 8, "0x10000b3", "0xb301", "0x12345678",
663  | |      "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }
     | |_________________________________________________________________________________________________- in this macro invocation
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_div` is not yet stable as a const fn
    --> library/core/src/num/uint_macros.rs:535:31
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
535  |  |                 Some(unsafe { intrinsics::unchecked_div(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
1833 |  |     }
1834 |  | }
1834 |  | }
     |  |_- in this expansion of `uint_impl!`
    ::: library/core/src/num/mod.rs:668:5
     |
     |
668  | /      uint_impl! { u64, u64, 64, 18446744073709551615, 12, "0xaa00000000006e1", "0x6e10aa",
669  | |      "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
670  | |      "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
671  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
672  | |      "", ""}
     | |____________- in this macro invocation
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_div` is not yet stable as a const fn
    --> library/core/src/num/uint_macros.rs:535:31
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
535  |  |                 Some(unsafe { intrinsics::unchecked_div(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
1833 |  |     }
1834 |  | }
1834 |  | }
     |  |_- in this expansion of `uint_impl!`
    ::: library/core/src/num/mod.rs:677:5
     |
     |
677  | /      uint_impl! { u128, u128, 128, 340282366920938463463374607431768211455, 16,
678  | |      "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
679  | |      "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
680  | |      "[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, \
683  | |        0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]",
684  | |       "", ""}
     | |_____________- in this macro invocation
     |
     |
     = help: Const-stable functions can only call other const-stable functions

error: `intrinsics::unchecked_div` is not yet stable as a const fn
    --> library/core/src/num/uint_macros.rs:535:31
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
535  |  |                 Some(unsafe { intrinsics::unchecked_div(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
1833 |  |     }
1834 |  | }
1834 |  | }
     |  |_- in this expansion of `uint_impl!`
    ::: library/core/src/num/mod.rs:705:5
     |
     |
705  | /      uint_impl! { usize, u64, 64, 18446744073709551615, 12, "0xaa00000000006e1", "0x6e10aa",
706  | |      "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
707  | |      "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
708  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
709  | |      usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
     | |_______________________________________________________________________- in this macro invocation
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_rem` is not yet stable as a const fn
    --> library/core/src/num/uint_macros.rs:586:31
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
586  |  |                 Some(unsafe { intrinsics::unchecked_rem(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
1833 |  |     }
1834 |  | }
1834 |  | }
     |  |_- in this expansion of `uint_impl!`
    ::: library/core/src/num/mod.rs:157:5
     |
     |
157  | /      uint_impl! { u8, u8, 8, 255, 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
158  | |      "[0x12]", "", "" }
     | |_______________________- in this macro invocation
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_rem` is not yet stable as a const fn
    --> library/core/src/num/uint_macros.rs:586:31
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
586  |  |                 Some(unsafe { intrinsics::unchecked_rem(self, rhs) })
     |  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
1833 |  |     }
1834 |  | }
1834 |  | }
     |  |_- in this expansion of `uint_impl!`
    ::: library/core/src/num/mod.rs:656:5
     |
     |
656  | /      uint_impl! { u16, u16, 16, 65535, 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
657  | |      "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     | |_____________________________________________- in this macro invocation
     = help: Const-stable functions can only call other const-stable functions


error: `intrinsics::unchecked_rem` is not yet stable as a const fn
    --> library/core/src/num/uint_macros.rs:586:31
     |
