plain
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.45
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: this arithmetic operation will overflow
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1775 |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `i8::MAX + 1_i8`, which would overflow
2222 |  |     }
2223 |  | }
     |  |_- in this expansion of `int_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:87:5
     |
87   | /      int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
88   | |      "[0x12]", "[0x12]", "", "" }
     |
     |
     = note: `#[deny(arithmetic_overflow)]` on by default

error: this arithmetic operation will overflow
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1805 |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `i8::MAX + 1_i8`, which would overflow
2222 |  |     }
2223 |  | }
     |  |_- in this expansion of `int_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:87:5
     |
87   | /      int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
88   | |      "[0x12]", "[0x12]", "", "" }


error: this arithmetic operation will overflow
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1835 |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `i8::MAX + 1_i8`, which would overflow
2222 |  |     }
2223 |  | }
     |  |_- in this expansion of `int_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:87:5
     |
87   | /      int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
88   | |      "[0x12]", "[0x12]", "", "" }


error: this arithmetic operation will overflow
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1775 |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `i16::MAX + 1_i16`, which would overflow
2222 |  |     }
2223 |  | }
     |  |_- in this expansion of `int_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:93:5
     |
93   | /      int_impl! { i16, i16, u16, 16, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
94   | |      "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }


error: this arithmetic operation will overflow
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1805 |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `i16::MAX + 1_i16`, which would overflow
2222 |  |     }
2223 |  | }
     |  |_- in this expansion of `int_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:93:5
     |
93   | /      int_impl! { i16, i16, u16, 16, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
94   | |      "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }


error: this arithmetic operation will overflow
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1835 |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `i16::MAX + 1_i16`, which would overflow
2222 |  |     }
2223 |  | }
     |  |_- in this expansion of `int_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:93:5
     |
93   | /      int_impl! { i16, i16, u16, 16, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
94   | |      "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }


error: this arithmetic operation will overflow
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1775 |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow
2222 |  |     }
2223 |  | }
     |  |_- in this expansion of `int_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:99:5
     |
99   | /      int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
100  | |      "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
101  | |      "[0x12, 0x34, 0x56, 0x78]", "", "" }


error: this arithmetic operation will overflow
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1805 |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow
2222 |  |     }
2223 |  | }
     |  |_- in this expansion of `int_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:99:5
     |
99   | /      int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
100  | |      "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
101  | |      "[0x12, 0x34, 0x56, 0x78]", "", "" }


error: this arithmetic operation will overflow
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1835 |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow
2222 |  |     }
2223 |  | }
     |  |_- in this expansion of `int_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:99:5
     |
99   | /      int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
100  | |      "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
101  | |      "[0x12, 0x34, 0x56, 0x78]", "", "" }


error: this arithmetic operation will overflow
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1775 |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `i64::MAX + 1_i64`, which would overflow
2222 |  |     }
2223 |  | }
     |  |_- in this expansion of `int_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:106:5
     |
106  | /      int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, 12,
107  | |      "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
108  | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
109  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }


error: this arithmetic operation will overflow
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1805 |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `i64::MAX + 1_i64`, which would overflow
2222 |  |     }
2223 |  | }
     |  |_- in this expansion of `int_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:106:5
     |
106  | /      int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, 12,
107  | |      "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
108  | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
109  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }


error: this arithmetic operation will overflow
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1835 |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `i64::MAX + 1_i64`, which would overflow
2222 |  |     }
2223 |  | }
     |  |_- in this expansion of `int_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:106:5
     |
106  | /      int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, 12,
107  | |      "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
108  | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
109  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }


error: this arithmetic operation will overflow
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1775 |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `i128::MAX + 1_i128`, which would overflow
2222 |  |     }
2223 |  | }
     |  |_- in this expansion of `int_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:114:5
     |
114  | /      int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
115  | |      170141183460469231731687303715884105727, 16,
116  | |      "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
117  | |      "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
120  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
121  | |        0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }


error: this arithmetic operation will overflow
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1805 |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `i128::MAX + 1_i128`, which would overflow
2222 |  |     }
2223 |  | }
     |  |_- in this expansion of `int_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:114:5
     |
114  | /      int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
115  | |      170141183460469231731687303715884105727, 16,
116  | |      "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
117  | |      "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
120  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
121  | |        0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }


error: this arithmetic operation will overflow
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1835 |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `i128::MAX + 1_i128`, which would overflow
2222 |  |     }
2223 |  | }
     |  |_- in this expansion of `int_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:114:5
     |
114  | /      int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
115  | |      170141183460469231731687303715884105727, 16,
116  | |      "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
117  | |      "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
120  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
121  | |        0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }


error: this arithmetic operation will overflow
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1775 |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `isize::MAX + 1_isize`, which would overflow
2222 |  |     }
2223 |  | }
     |  |_- in this expansion of `int_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:144:5
     |
144  | /      int_impl! { isize, i64, usize, 64, -9223372036854775808, 9223372036854775807,
145  | |      12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
146  | |       "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
147  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
148  | |       usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }


error: this arithmetic operation will overflow
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1805 |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `isize::MAX + 1_isize`, which would overflow
2222 |  |     }
2223 |  | }
     |  |_- in this expansion of `int_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:144:5
     |
144  | /      int_impl! { isize, i64, usize, 64, -9223372036854775808, 9223372036854775807,
145  | |      12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
146  | |       "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
147  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
148  | |       usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }


error: this arithmetic operation will overflow
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1835 |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `isize::MAX + 1_isize`, which would overflow
2222 |  |     }
2223 |  | }
     |  |_- in this expansion of `int_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:144:5
     |
144  | /      int_impl! { isize, i64, usize, 64, -9223372036854775808, 9223372036854775807,
145  | |      12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
146  | |       "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
147  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
148  | |       usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }


error: this arithmetic operation will overflow
    --> library/core/src/num/uint_macros.rs:665:29
1    |  / macro_rules! uint_impl {
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
665  |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
2048 |  |     }
2049 |  | }
     |  |_- in this expansion of `uint_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:156:5
     |
156  | /      uint_impl! { u8, u8, 8, 255, 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
157  | |      "[0x12]", "", "" }


error: this arithmetic operation will overflow
    --> library/core/src/num/uint_macros.rs:695:29
1    |  / macro_rules! uint_impl {
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
695  |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
2048 |  |     }
2049 |  | }
     |  |_- in this expansion of `uint_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:156:5
     |
156  | /      uint_impl! { u8, u8, 8, 255, 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
157  | |      "[0x12]", "", "" }


error: this arithmetic operation will overflow
    --> library/core/src/num/uint_macros.rs:725:29
1    |  / macro_rules! uint_impl {
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
725  |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
2048 |  |     }
2049 |  | }
     |  |_- in this expansion of `uint_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:156:5
     |
156  | /      uint_impl! { u8, u8, 8, 255, 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
157  | |      "[0x12]", "", "" }


error: this arithmetic operation will overflow
    --> library/core/src/num/uint_macros.rs:665:29
1    |  / macro_rules! uint_impl {
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
665  |  |                     let _ = Self::MAX + 1; // overflow in order to panic in debug mode
     |  |                             ^^^^^^^^^^^^^ attempt to compute `u16::MAX + 1_u16`, which would overflow
2048 |  |     }
2049 |  | }
     |  |_- in this expansion of `uint_impl!`
     | 
     | 
    ::: library/core/src/num/mod.rs:689:5
     |
689  | /      uint_impl! { u16, u16, 16, 65535, 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
690  | |      "[0x34, 0x12]", "[0x12, 0x34]", "", "" }

