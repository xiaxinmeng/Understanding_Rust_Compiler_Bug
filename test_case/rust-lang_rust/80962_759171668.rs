plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: attribute should be applied to a macro
    --> library/core/src/num/int_macros.rs:517:9
     |
1    |   / macro_rules! int_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |   |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
517  |   |         #[allow_internal_unstable(const_int_unchecked_arith)]
     |   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...      |
521  | / |         pub const fn checked_div(self, rhs: Self) -> Option<Self> {
522  | | |             if unlikely!(rhs == 0 || (self == Self::MIN && rhs == -1)) {
523  | | |                 None
524  | | |             } else {
527  | | |             }
528  | | |         }
528  | | |         }
     | |_|_________- not a macro
2009 |   |     }
2010 |   | }
2010 |   | }
     |   |_- in this expansion of `int_impl!`
    ::: library/core/src/num/mod.rs:91:5
     |
     |
91   | /       int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
92   | |       "[0x12]", "[0x12]", "", "" }
     | |__________________________________- in this macro invocation
error: attribute should be applied to a macro
    --> library/core/src/num/int_macros.rs:570:9
     |
     |
1    |   / macro_rules! int_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |   |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
570  |   |         #[allow_internal_unstable(const_int_unchecked_arith)]
     |   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...      |
574  | / |         pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
575  | | |             if unlikely!(rhs == 0 || (self == Self::MIN && rhs == -1)) {
576  | | |                 None
577  | | |             } else {
580  | | |             }
581  | | |         }
581  | | |         }
     | |_|_________- not a macro
2009 |   |     }
2010 |   | }
2010 |   | }
     |   |_- in this expansion of `int_impl!`
    ::: library/core/src/num/mod.rs:91:5
     |
     |
91   | /       int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
92   | |       "[0x12]", "[0x12]", "", "" }
     | |__________________________________- in this macro invocation
error: attribute should be applied to a macro
    --> library/core/src/num/int_macros.rs:517:9
     |
     |
1    |   / macro_rules! int_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |   |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
517  |   |         #[allow_internal_unstable(const_int_unchecked_arith)]
     |   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...      |
521  | / |         pub const fn checked_div(self, rhs: Self) -> Option<Self> {
522  | | |             if unlikely!(rhs == 0 || (self == Self::MIN && rhs == -1)) {
523  | | |                 None
524  | | |             } else {
527  | | |             }
528  | | |         }
528  | | |         }
     | |_|_________- not a macro
2009 |   |     }
2010 |   | }
2010 |   | }
     |   |_- in this expansion of `int_impl!`
    ::: library/core/src/num/mod.rs:97:5
     |
     |
97   | /       int_impl! { i16, i16, u16, 16, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
98   | |       "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     | |________________________________________________________- in this macro invocation
error: attribute should be applied to a macro
    --> library/core/src/num/int_macros.rs:570:9
     |
     |
1    |   / macro_rules! int_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |   |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
570  |   |         #[allow_internal_unstable(const_int_unchecked_arith)]
     |   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...      |
574  | / |         pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
575  | | |             if unlikely!(rhs == 0 || (self == Self::MIN && rhs == -1)) {
576  | | |                 None
577  | | |             } else {
580  | | |             }
581  | | |         }
581  | | |         }
     | |_|_________- not a macro
2009 |   |     }
2010 |   | }
2010 |   | }
     |   |_- in this expansion of `int_impl!`
    ::: library/core/src/num/mod.rs:97:5
     |
     |
97   | /       int_impl! { i16, i16, u16, 16, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
98   | |       "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     | |________________________________________________________- in this macro invocation
error: attribute should be applied to a macro
    --> library/core/src/num/int_macros.rs:517:9
     |
     |
1    |   / macro_rules! int_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |   |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
517  |   |         #[allow_internal_unstable(const_int_unchecked_arith)]
     |   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...      |
521  | / |         pub const fn checked_div(self, rhs: Self) -> Option<Self> {
522  | | |             if unlikely!(rhs == 0 || (self == Self::MIN && rhs == -1)) {
523  | | |                 None
524  | | |             } else {
527  | | |             }
528  | | |         }
528  | | |         }
     | |_|_________- not a macro
2009 |   |     }
2010 |   | }
2010 |   | }
     |   |_- in this expansion of `int_impl!`
    ::: library/core/src/num/mod.rs:103:5
     |
     |
103  | /       int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
104  | |       "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
105  | |       "[0x12, 0x34, 0x56, 0x78]", "", "" }
     | |__________________________________________- in this macro invocation
error: attribute should be applied to a macro
    --> library/core/src/num/int_macros.rs:570:9
     |
     |
1    |   / macro_rules! int_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |   |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
570  |   |         #[allow_internal_unstable(const_int_unchecked_arith)]
     |   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...      |
574  | / |         pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
575  | | |             if unlikely!(rhs == 0 || (self == Self::MIN && rhs == -1)) {
576  | | |                 None
577  | | |             } else {
580  | | |             }
581  | | |         }
581  | | |         }
     | |_|_________- not a macro
2009 |   |     }
2010 |   | }
2010 |   | }
     |   |_- in this expansion of `int_impl!`
    ::: library/core/src/num/mod.rs:103:5
     |
     |
103  | /       int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
104  | |       "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
105  | |       "[0x12, 0x34, 0x56, 0x78]", "", "" }
     | |__________________________________________- in this macro invocation
error: attribute should be applied to a macro
    --> library/core/src/num/int_macros.rs:517:9
     |
     |
1    |   / macro_rules! int_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |   |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
517  |   |         #[allow_internal_unstable(const_int_unchecked_arith)]
     |   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...      |
521  | / |         pub const fn checked_div(self, rhs: Self) -> Option<Self> {
522  | | |             if unlikely!(rhs == 0 || (self == Self::MIN && rhs == -1)) {
523  | | |                 None
524  | | |             } else {
527  | | |             }
528  | | |         }
528  | | |         }
     | |_|_________- not a macro
2009 |   |     }
2010 |   | }
2010 |   | }
     |   |_- in this expansion of `int_impl!`
    ::: library/core/src/num/mod.rs:110:5
     |
     |
110  | /       int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, 12,
111  | |       "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
112  | |       "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
113  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
     | |__________________________________________________________________- in this macro invocation
error: attribute should be applied to a macro
    --> library/core/src/num/int_macros.rs:570:9
     |
     |
1    |   / macro_rules! int_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |   |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
570  |   |         #[allow_internal_unstable(const_int_unchecked_arith)]
     |   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...      |
574  | / |         pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
575  | | |             if unlikely!(rhs == 0 || (self == Self::MIN && rhs == -1)) {
576  | | |                 None
577  | | |             } else {
580  | | |             }
581  | | |         }
581  | | |         }
     | |_|_________- not a macro
2009 |   |     }
2010 |   | }
2010 |   | }
     |   |_- in this expansion of `int_impl!`
    ::: library/core/src/num/mod.rs:110:5
     |
     |
110  | /       int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, 12,
111  | |       "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
112  | |       "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
113  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
     | |__________________________________________________________________- in this macro invocation
error: attribute should be applied to a macro
    --> library/core/src/num/int_macros.rs:517:9
     |
     |
1    |   / macro_rules! int_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |   |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
517  |   |         #[allow_internal_unstable(const_int_unchecked_arith)]
     |   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...      |
521  | / |         pub const fn checked_div(self, rhs: Self) -> Option<Self> {
522  | | |             if unlikely!(rhs == 0 || (self == Self::MIN && rhs == -1)) {
523  | | |                 None
524  | | |             } else {
527  | | |             }
528  | | |         }
528  | | |         }
     | |_|_________- not a macro
2009 |   |     }
2010 |   | }
2010 |   | }
     |   |_- in this expansion of `int_impl!`
    ::: library/core/src/num/mod.rs:118:5
     |
     |
118  | /       int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
119  | |       170141183460469231731687303715884105727, 16,
120  | |       "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
121  | |       "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
124  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
125  | |         0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
     | |__________________________________________________________________- in this macro invocation
error: attribute should be applied to a macro
    --> library/core/src/num/int_macros.rs:570:9
     |
     |
1    |   / macro_rules! int_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |   |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
570  |   |         #[allow_internal_unstable(const_int_unchecked_arith)]
     |   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...      |
574  | / |         pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
575  | | |             if unlikely!(rhs == 0 || (self == Self::MIN && rhs == -1)) {
576  | | |                 None
577  | | |             } else {
580  | | |             }
581  | | |         }
581  | | |         }
     | |_|_________- not a macro
2009 |   |     }
2010 |   | }
2010 |   | }
     |   |_- in this expansion of `int_impl!`
    ::: library/core/src/num/mod.rs:118:5
     |
     |
118  | /       int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
119  | |       170141183460469231731687303715884105727, 16,
120  | |       "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
121  | |       "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
124  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
125  | |         0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
     | |__________________________________________________________________- in this macro invocation
error: attribute should be applied to a macro
    --> library/core/src/num/int_macros.rs:517:9
     |
     |
1    |   / macro_rules! int_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |   |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
517  |   |         #[allow_internal_unstable(const_int_unchecked_arith)]
     |   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...      |
521  | / |         pub const fn checked_div(self, rhs: Self) -> Option<Self> {
522  | | |             if unlikely!(rhs == 0 || (self == Self::MIN && rhs == -1)) {
523  | | |                 None
524  | | |             } else {
527  | | |             }
528  | | |         }
528  | | |         }
     | |_|_________- not a macro
2009 |   |     }
2010 |   | }
2010 |   | }
     |   |_- in this expansion of `int_impl!`
    ::: library/core/src/num/mod.rs:148:5
     |
     |
148  | /       int_impl! { isize, i64, usize, 64, -9223372036854775808, 9223372036854775807,
149  | |       12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
150  | |        "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
151  | |        "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
152  | |        usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
     | |_________________________________________________________________________- in this macro invocation
error: attribute should be applied to a macro
    --> library/core/src/num/int_macros.rs:570:9
     |
     |
1    |   / macro_rules! int_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |   |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
570  |   |         #[allow_internal_unstable(const_int_unchecked_arith)]
     |   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...      |
574  | / |         pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
575  | | |             if unlikely!(rhs == 0 || (self == Self::MIN && rhs == -1)) {
576  | | |                 None
577  | | |             } else {
580  | | |             }
581  | | |         }
581  | | |         }
     | |_|_________- not a macro
2009 |   |     }
2010 |   | }
2010 |   | }
     |   |_- in this expansion of `int_impl!`
    ::: library/core/src/num/mod.rs:148:5
     |
     |
148  | /       int_impl! { isize, i64, usize, 64, -9223372036854775808, 9223372036854775807,
149  | |       12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
150  | |        "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
151  | |        "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
152  | |        usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
     | |_________________________________________________________________________- in this macro invocation
error: attribute should be applied to a macro
    --> library/core/src/num/uint_macros.rs:526:9
     |
     |
1    |   / macro_rules! uint_impl {
2    |   |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr,
3    |   |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
526  |   |         #[allow_internal_unstable(const_int_unchecked_arith)]
     |   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...      |
530  | / |         pub const fn checked_div(self, rhs: Self) -> Option<Self> {
531  | | |             if unlikely!(rhs == 0) {
532  | | |                 None
533  | | |             } else {
537  | | |             }
538  | | |         }
538  | | |         }
     | |_|_________- not a macro
1835 |   |     }
1836 |   | }
1836 |   | }
     |   |_- in this expansion of `uint_impl!`
    ::: library/core/src/num/mod.rs:157:5
     |
     |
157  | /       uint_impl! { u8, u8, 8, 255, 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
158  | |       "[0x12]", "", "" }
     | |________________________- in this macro invocation
error: attribute should be applied to a macro
    --> library/core/src/num/uint_macros.rs:578:9
     |
     |
1    |   / macro_rules! uint_impl {
2    |   |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr,
3    |   |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
578  |   |         #[allow_internal_unstable(const_int_unchecked_arith)]
     |   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...      |
582  | / |         pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
583  | | |             if unlikely!(rhs == 0) {
584  | | |                 None
585  | | |             } else {
589  | | |             }
590  | | |         }
590  | | |         }
     | |_|_________- not a macro
1835 |   |     }
1836 |   | }
1836 |   | }
     |   |_- in this expansion of `uint_impl!`
    ::: library/core/src/num/mod.rs:157:5
     |
     |
157  | /       uint_impl! { u8, u8, 8, 255, 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
158  | |       "[0x12]", "", "" }
     | |________________________- in this macro invocation
error: attribute should be applied to a macro
    --> library/core/src/num/uint_macros.rs:526:9
     |
     |
1    |   / macro_rules! uint_impl {
2    |   |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr,
3    |   |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
526  |   |         #[allow_internal_unstable(const_int_unchecked_arith)]
     |   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...      |
530  | / |         pub const fn checked_div(self, rhs: Self) -> Option<Self> {
531  | | |             if unlikely!(rhs == 0) {
532  | | |                 None
533  | | |             } else {
537  | | |             }
538  | | |         }
538  | | |         }
     | |_|_________- not a macro
1835 |   |     }
1836 |   | }
1836 |   | }
     |   |_- in this expansion of `uint_impl!`
    ::: library/core/src/num/mod.rs:656:5
     |
     |
656  | /       uint_impl! { u16, u16, 16, 65535, 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
657  | |       "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     | |______________________________________________- in this macro invocation
error: attribute should be applied to a macro
    --> library/core/src/num/uint_macros.rs:578:9
     |
     |
1    |   / macro_rules! uint_impl {
2    |   |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr,
3    |   |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
578  |   |         #[allow_internal_unstable(const_int_unchecked_arith)]
     |   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...      |
582  | / |         pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
583  | | |             if unlikely!(rhs == 0) {
584  | | |                 None
585  | | |             } else {
589  | | |             }
590  | | |         }
590  | | |         }
     | |_|_________- not a macro
1835 |   |     }
1836 |   | }
1836 |   | }
     |   |_- in this expansion of `uint_impl!`
    ::: library/core/src/num/mod.rs:656:5
     |
     |
656  | /       uint_impl! { u16, u16, 16, 65535, 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
