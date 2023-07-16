plain
   Compiling libc v0.2.98
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.47
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0658]: `transmute` is not allowed in constant functions
    |
    |
748 |         unsafe { mem::transmute(self) }
    |
    = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
    = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
    = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
    = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
    |
    |
793 |         unsafe { mem::transmute(v) }
    |
    = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
    = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
    = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
    = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
    |
    |
762 |         unsafe { mem::transmute(self) }
    |
    = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
    = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
    = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
    = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
    |
    |
807 |         unsafe { mem::transmute(v) }
    |
    = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
    = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
    = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
    = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2105 |  |             unsafe { mem::transmute(self) }
...     |
2237 |  |     }
2238 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:94:5
     |
94   | /      int_impl! { i8, i8, u8, 8, 7, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
95   | |      "[0x12]", "[0x12]", "", "" }
     |
     = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2105 |  |             unsafe { mem::transmute(self) }
...     |
2237 |  |     }
2238 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:100:5
     |
100  | /      int_impl! { i16, i16, u16, 16, 15, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
101  | |      "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     |
     = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2105 |  |             unsafe { mem::transmute(self) }
...     |
2237 |  |     }
2238 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:106:5
     |
106  | /      int_impl! { i32, i32, u32, 32, 31, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
107  | |      "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
108  | |      "[0x12, 0x34, 0x56, 0x78]", "", "" }
     |
     = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2105 |  |             unsafe { mem::transmute(self) }
...     |
2237 |  |     }
2238 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:113:5
     |
113  | /      int_impl! { i64, i64, u64, 64, 63, -9223372036854775808, 9223372036854775807, 12,
114  | |      "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
115  | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
116  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
     |
     = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2105 |  |             unsafe { mem::transmute(self) }
...     |
2237 |  |     }
2238 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:121:5
     |
121  | /      int_impl! { i128, i128, u128, 128, 127, -170141183460469231731687303715884105728,
122  | |      170141183460469231731687303715884105727, 16,
123  | |      "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
124  | |      "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
127  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
128  | |        0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
     |
     = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2105 |  |             unsafe { mem::transmute(self) }
...     |
2237 |  |     }
2238 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:151:5
     |
151  | /      int_impl! { isize, i64, usize, 64, 63, -9223372036854775808, 9223372036854775807,
152  | |      12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
153  | |       "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
154  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
155  | |       usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
     |
     = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2209 |  |             unsafe { mem::transmute(bytes) }
...     |
2237 |  |     }
2238 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:94:5
     |
94   | /      int_impl! { i8, i8, u8, 8, 7, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
95   | |      "[0x12]", "[0x12]", "", "" }
     |
     = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2209 |  |             unsafe { mem::transmute(bytes) }
...     |
2237 |  |     }
2238 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:100:5
     |
100  | /      int_impl! { i16, i16, u16, 16, 15, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
101  | |      "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     |
     = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2209 |  |             unsafe { mem::transmute(bytes) }
...     |
2237 |  |     }
2238 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:106:5
     |
106  | /      int_impl! { i32, i32, u32, 32, 31, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
107  | |      "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
108  | |      "[0x12, 0x34, 0x56, 0x78]", "", "" }
     |
     = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2209 |  |             unsafe { mem::transmute(bytes) }
...     |
2237 |  |     }
2238 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:113:5
     |
113  | /      int_impl! { i64, i64, u64, 64, 63, -9223372036854775808, 9223372036854775807, 12,
114  | |      "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
115  | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
116  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
     |
     = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2209 |  |             unsafe { mem::transmute(bytes) }
...     |
2237 |  |     }
2238 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:121:5
     |
121  | /      int_impl! { i128, i128, u128, 128, 127, -170141183460469231731687303715884105728,
122  | |      170141183460469231731687303715884105727, 16,
123  | |      "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
124  | |      "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
127  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
128  | |        0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
     |
     = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2209 |  |             unsafe { mem::transmute(bytes) }
...     |
2237 |  |     }
2238 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:151:5
     |
151  | /      int_impl! { isize, i64, usize, 64, 63, -9223372036854775808, 9223372036854775807,
152  | |      12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
153  | |       "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
154  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
155  | |       usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
     |
     = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
    --> library/core/src/num/uint_macros.rs:1934:22
1    |  / macro_rules! uint_impl {
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1934 |  |             unsafe { mem::transmute(self) }
...     |
2062 |  |     }
2063 |  | }
     |  |_- in this expansion of `uint_impl!`
     |  |_- in this expansion of `uint_impl!`
     | 
    ::: library/core/src/num/mod.rs:163:5
     |
163  | /      uint_impl! { u8, u8, 8, 255, 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
164  | |      "[0x12]", "", "" }
     |
     = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
    --> library/core/src/num/uint_macros.rs:1934:22
1    |  / macro_rules! uint_impl {
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1934 |  |             unsafe { mem::transmute(self) }
...     |
2062 |  |     }
2063 |  | }
     |  |_- in this expansion of `uint_impl!`
     |  |_- in this expansion of `uint_impl!`
     | 
    ::: library/core/src/num/mod.rs:696:5
     |
696  | /      uint_impl! { u16, u16, 16, 65535, 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
697  | |      "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     |
     = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
    --> library/core/src/num/uint_macros.rs:1934:22
1    |  / macro_rules! uint_impl {
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1934 |  |             unsafe { mem::transmute(self) }
...     |
2062 |  |     }
2063 |  | }
     |  |_- in this expansion of `uint_impl!`
     |  |_- in this expansion of `uint_impl!`
     | 
    ::: library/core/src/num/mod.rs:702:5
     |
702  | /      uint_impl! { u32, u32, 32, 4294967295, 8, "0x10000b3", "0xb301", "0x12345678",
703  | |      "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }
     |
     = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
    --> library/core/src/num/uint_macros.rs:1934:22
1    |  / macro_rules! uint_impl {
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1934 |  |             unsafe { mem::transmute(self) }
...     |
2062 |  |     }
2063 |  | }
     |  |_- in this expansion of `uint_impl!`
     |  |_- in this expansion of `uint_impl!`
     | 
    ::: library/core/src/num/mod.rs:708:5
     |
708  | /      uint_impl! { u64, u64, 64, 18446744073709551615, 12, "0xaa00000000006e1", "0x6e10aa",
709  | |      "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
710  | |      "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
711  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
712  | |      "", ""}
     |
     = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
    --> library/core/src/num/uint_macros.rs:1934:22
1    |  / macro_rules! uint_impl {
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1934 |  |             unsafe { mem::transmute(self) }
...     |
2062 |  |     }
2063 |  | }
     |  |_- in this expansion of `uint_impl!`
     |  |_- in this expansion of `uint_impl!`
     | 
    ::: library/core/src/num/mod.rs:717:5
     |
717  | /      uint_impl! { u128, u128, 128, 340282366920938463463374607431768211455, 16,
718  | |      "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
719  | |      "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
720  | |      "[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, \
723  | |        0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]",
724  | |       "", ""}
     | |_____________- in this macro invocation
     |
     |
     = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
     = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
     = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
---

error[E0658]: unions in const fn are unstable
  --> library/core/src/ptr/metadata.rs:97:14
   |
97 |     unsafe { PtrRepr { const_ptr: ptr }.components.metadata }
   |
   = note: see issue #51909 <https://github.com/rust-lang/rust/issues/51909> for more information
   = help: add `#![feature(const_fn_union)]` to the crate attributes to enable


error[E0658]: unions in const fn are unstable
   --> library/core/src/ptr/metadata.rs:117:14
    |
117 |     unsafe { PtrRepr { components: PtrComponents { data_address, metadata } }.const_ptr }
    |
    = note: see issue #51909 <https://github.com/rust-lang/rust/issues/51909> for more information
    = help: add `#![feature(const_fn_union)]` to the crate attributes to enable


error[E0658]: unions in const fn are unstable
   --> library/core/src/ptr/metadata.rs:134:14
    |
134 |     unsafe { PtrRepr { components: PtrComponents { data_address, metadata } }.mut_ptr }
    |
    = note: see issue #51909 <https://github.com/rust-lang/rust/issues/51909> for more information
    = help: add `#![feature(const_fn_union)]` to the crate attributes to enable


error[E0658]: unions in const fn are unstable
   --> library/core/src/slice/mod.rs:111:18
    |
111 |         unsafe { crate::ptr::PtrRepr { const_ptr: self }.components.metadata }
    |
    = note: see issue #51909 <https://github.com/rust-lang/rust/issues/51909> for more information
    = help: add `#![feature(const_fn_union)]` to the crate attributes to enable


error[E0658]: `transmute` is not allowed in constant functions
    |
    |
236 |         unsafe { mem::transmute(self) }
    |
    = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
    = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
    = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
    = note: `transmute` is only allowed in constants and statics for now

error[E0658]: `transmute` is not allowed in constant functions
    |
    |
163 |     unsafe { mem::transmute(v) }
    |
    = note: see issue #53605 <https://github.com/rust-lang/rust/issues/53605> for more information
    = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
    = help: add `#![feature(const_fn_transmute)]` to the crate attributes to enable
    = note: `transmute` is only allowed in constants and statics for now
For more information about this error, try `rustc --explain E0658`.
error: could not compile `core` due to 35 previous errors
Build completed unsuccessfully in 0:00:09
