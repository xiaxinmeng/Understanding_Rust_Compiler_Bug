plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: associated function has missing stability attribute
    --> library/core/src/num/int_macros.rs:2545:9
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2545 |  |         pub const fn is_even(self) -> bool { self % 2 == 0 }
...     |
2765 |  |     }
2766 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:198:5
     |
198  | /      int_impl! { i8, i8, u8, 8, 7, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
199  | |      "[0x12]", "[0x12]", "", "" }

error: associated function has missing stability attribute
    --> library/core/src/num/int_macros.rs:2559:9
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2559 |  |         pub const fn is_odd(self) -> bool { !self.is_even() }
...     |
2765 |  |     }
2766 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:198:5
     |
198  | /      int_impl! { i8, i8, u8, 8, 7, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
199  | |      "[0x12]", "[0x12]", "", "" }

error: associated function has missing stability attribute
    --> library/core/src/num/int_macros.rs:2545:9
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2545 |  |         pub const fn is_even(self) -> bool { self % 2 == 0 }
...     |
2765 |  |     }
2766 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:203:5
     |
203  | /      int_impl! { i16, i16, u16, 16, 15, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
204  | |      "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }

error: associated function has missing stability attribute
    --> library/core/src/num/int_macros.rs:2559:9
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2559 |  |         pub const fn is_odd(self) -> bool { !self.is_even() }
...     |
2765 |  |     }
2766 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:203:5
     |
203  | /      int_impl! { i16, i16, u16, 16, 15, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
204  | |      "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }

error: associated function has missing stability attribute
    --> library/core/src/num/int_macros.rs:2545:9
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2545 |  |         pub const fn is_even(self) -> bool { self % 2 == 0 }
...     |
2765 |  |     }
2766 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:208:5
     |
208  | /      int_impl! { i32, i32, u32, 32, 31, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
209  | |      "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
210  | |      "[0x12, 0x34, 0x56, 0x78]", "", "" }

error: associated function has missing stability attribute
    --> library/core/src/num/int_macros.rs:2559:9
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2559 |  |         pub const fn is_odd(self) -> bool { !self.is_even() }
...     |
2765 |  |     }
2766 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:208:5
     |
208  | /      int_impl! { i32, i32, u32, 32, 31, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
209  | |      "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
210  | |      "[0x12, 0x34, 0x56, 0x78]", "", "" }

error: associated function has missing stability attribute
    --> library/core/src/num/int_macros.rs:2545:9
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2545 |  |         pub const fn is_even(self) -> bool { self % 2 == 0 }
...     |
2765 |  |     }
2766 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:214:5
     |
214  | /      int_impl! { i64, i64, u64, 64, 63, -9223372036854775808, 9223372036854775807, 12,
215  | |      "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
216  | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
217  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }

error: associated function has missing stability attribute
    --> library/core/src/num/int_macros.rs:2559:9
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2559 |  |         pub const fn is_odd(self) -> bool { !self.is_even() }
...     |
2765 |  |     }
2766 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:214:5
     |
214  | /      int_impl! { i64, i64, u64, 64, 63, -9223372036854775808, 9223372036854775807, 12,
215  | |      "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
216  | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
217  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }

error: associated function has missing stability attribute
    --> library/core/src/num/int_macros.rs:2545:9
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2545 |  |         pub const fn is_even(self) -> bool { self % 2 == 0 }
...     |
2765 |  |     }
2766 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:221:5
     |
221  | /      int_impl! { i128, i128, u128, 128, 127, -170141183460469231731687303715884105728,
222  | |      170141183460469231731687303715884105727, 16,
223  | |      "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
224  | |      "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
227  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
228  | |        0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }

error: associated function has missing stability attribute
    --> library/core/src/num/int_macros.rs:2559:9
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2559 |  |         pub const fn is_odd(self) -> bool { !self.is_even() }
...     |
2765 |  |     }
2766 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:221:5
     |
221  | /      int_impl! { i128, i128, u128, 128, 127, -170141183460469231731687303715884105728,
222  | |      170141183460469231731687303715884105727, 16,
223  | |      "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
224  | |      "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
227  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
228  | |        0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }

error: associated function has missing stability attribute
    --> library/core/src/num/int_macros.rs:2545:9
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2545 |  |         pub const fn is_even(self) -> bool { self % 2 == 0 }
...     |
2765 |  |     }
2766 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:248:5
     |
248  | /      int_impl! { isize, i64, usize, 64, 63, -9223372036854775808, 9223372036854775807,
249  | |      12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
250  | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
251  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
252  | |      usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }

error: associated function has missing stability attribute
    --> library/core/src/num/int_macros.rs:2559:9
     |
     |
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
2559 |  |         pub const fn is_odd(self) -> bool { !self.is_even() }
...     |
2765 |  |     }
2766 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:248:5
     |
248  | /      int_impl! { isize, i64, usize, 64, 63, -9223372036854775808, 9223372036854775807,
249  | |      12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
250  | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
251  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
252  | |      usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }

error: associated function has missing stability attribute
    --> library/core/src/num/uint_macros.rs:2259:9
     |
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $NonZeroT:ident,
3    |  |         $BITS:expr, $MaxV:expr,
4    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
...     |
2259 |  |         pub const fn is_even(self) -> bool { self % 2 == 0 }
...     |
2475 |  |     }
2476 |  | }
     |  |_- in this expansion of `uint_impl!`
     |  |_- in this expansion of `uint_impl!`
     |
    ::: library/core/src/num/mod.rs:259:5
     |
259  | /      uint_impl! { u8, u8, i8, NonZeroU8, 8, 255, 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
260  | |      "[0x12]", "", "" }

error: associated function has missing stability attribute
    --> library/core/src/num/uint_macros.rs:2273:9
     |
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $NonZeroT:ident,
3    |  |         $BITS:expr, $MaxV:expr,
4    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
...     |
2273 |  |         pub const fn is_odd(self) -> bool { !self.is_even() }
...     |
2475 |  |     }
2476 |  | }
     |  |_- in this expansion of `uint_impl!`
     |  |_- in this expansion of `uint_impl!`
     |
    ::: library/core/src/num/mod.rs:259:5
     |
259  | /      uint_impl! { u8, u8, i8, NonZeroU8, 8, 255, 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
260  | |      "[0x12]", "", "" }

error: associated function has missing stability attribute
    --> library/core/src/num/uint_macros.rs:2259:9
     |
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $NonZeroT:ident,
3    |  |         $BITS:expr, $MaxV:expr,
4    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
...     |
2259 |  |         pub const fn is_even(self) -> bool { self % 2 == 0 }
...     |
2475 |  |     }
2476 |  | }
     |  |_- in this expansion of `uint_impl!`
     |  |_- in this expansion of `uint_impl!`
     |
    ::: library/core/src/num/mod.rs:812:5
     |
812  | /      uint_impl! { u16, u16, i16, NonZeroU16, 16, 65535, 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
813  | |      "[0x34, 0x12]", "[0x12, 0x34]", "", "" }

error: associated function has missing stability attribute
    --> library/core/src/num/uint_macros.rs:2273:9
     |
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $NonZeroT:ident,
3    |  |         $BITS:expr, $MaxV:expr,
4    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
...     |
2273 |  |         pub const fn is_odd(self) -> bool { !self.is_even() }
...     |
2475 |  |     }
2476 |  | }
     |  |_- in this expansion of `uint_impl!`
     |  |_- in this expansion of `uint_impl!`
     |
    ::: library/core/src/num/mod.rs:812:5
     |
812  | /      uint_impl! { u16, u16, i16, NonZeroU16, 16, 65535, 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
813  | |      "[0x34, 0x12]", "[0x12, 0x34]", "", "" }

error: associated function has missing stability attribute
    --> library/core/src/num/uint_macros.rs:2259:9
     |
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $NonZeroT:ident,
3    |  |         $BITS:expr, $MaxV:expr,
4    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
...     |
2259 |  |         pub const fn is_even(self) -> bool { self % 2 == 0 }
...     |
2475 |  |     }
2476 |  | }
     |  |_- in this expansion of `uint_impl!`
     |  |_- in this expansion of `uint_impl!`
     |
    ::: library/core/src/num/mod.rs:843:5
     |
843  | /      uint_impl! { u32, u32, i32, NonZeroU32, 32, 4294967295, 8, "0x10000b3", "0xb301", "0x12345678",
844  | |      "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }

error: associated function has missing stability attribute
    --> library/core/src/num/uint_macros.rs:2273:9
     |
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $NonZeroT:ident,
3    |  |         $BITS:expr, $MaxV:expr,
4    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
...     |
2273 |  |         pub const fn is_odd(self) -> bool { !self.is_even() }
...     |
2475 |  |     }
2476 |  | }
     |  |_- in this expansion of `uint_impl!`
     |  |_- in this expansion of `uint_impl!`
     |
    ::: library/core/src/num/mod.rs:843:5
     |
843  | /      uint_impl! { u32, u32, i32, NonZeroU32, 32, 4294967295, 8, "0x10000b3", "0xb301", "0x12345678",
844  | |      "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }

error: associated function has missing stability attribute
    --> library/core/src/num/uint_macros.rs:2259:9
     |
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $NonZeroT:ident,
3    |  |         $BITS:expr, $MaxV:expr,
4    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
...     |
2259 |  |         pub const fn is_even(self) -> bool { self % 2 == 0 }
...     |
2475 |  |     }
2476 |  | }
     |  |_- in this expansion of `uint_impl!`
     |  |_- in this expansion of `uint_impl!`
     |
    ::: library/core/src/num/mod.rs:849:5
     |
849  | /      uint_impl! { u64, u64, i64, NonZeroU64, 64, 18446744073709551615, 12, "0xaa00000000006e1", "0x6e10aa",
850  | |      "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
851  | |      "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
852  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
853  | |      "", ""}

error: associated function has missing stability attribute
    --> library/core/src/num/uint_macros.rs:2273:9
     |
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $NonZeroT:ident,
3    |  |         $BITS:expr, $MaxV:expr,
4    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
...     |
2273 |  |         pub const fn is_odd(self) -> bool { !self.is_even() }
...     |
2475 |  |     }
2476 |  | }
     |  |_- in this expansion of `uint_impl!`
     |  |_- in this expansion of `uint_impl!`
     |
    ::: library/core/src/num/mod.rs:849:5
     |
849  | /      uint_impl! { u64, u64, i64, NonZeroU64, 64, 18446744073709551615, 12, "0xaa00000000006e1", "0x6e10aa",
850  | |      "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
851  | |      "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
852  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
853  | |      "", ""}

error: associated function has missing stability attribute
    --> library/core/src/num/uint_macros.rs:2259:9
     |
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $NonZeroT:ident,
3    |  |         $BITS:expr, $MaxV:expr,
4    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
...     |
2259 |  |         pub const fn is_even(self) -> bool { self % 2 == 0 }
...     |
2475 |  |     }
2476 |  | }
     |  |_- in this expansion of `uint_impl!`
     |  |_- in this expansion of `uint_impl!`
     |
    ::: library/core/src/num/mod.rs:858:5
     |
858  | /      uint_impl! { u128, u128, i128, NonZeroU128, 128, 340282366920938463463374607431768211455, 16,
859  | |      "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
860  | |      "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
861  | |      "[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, \
864  | |        0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]",
865  | |       "", ""}
     | |_____________- in this macro invocation


error: associated function has missing stability attribute
    --> library/core/src/num/uint_macros.rs:2273:9
     |
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $NonZeroT:ident,
3    |  |         $BITS:expr, $MaxV:expr,
4    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
...     |
2273 |  |         pub const fn is_odd(self) -> bool { !self.is_even() }
...     |
2475 |  |     }
2476 |  | }
     |  |_- in this expansion of `uint_impl!`
     |  |_- in this expansion of `uint_impl!`
     |
    ::: library/core/src/num/mod.rs:858:5
     |
858  | /      uint_impl! { u128, u128, i128, NonZeroU128, 128, 340282366920938463463374607431768211455, 16,
859  | |      "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
860  | |      "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
861  | |      "[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, \
864  | |        0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]",
865  | |       "", ""}
     | |_____________- in this macro invocation

