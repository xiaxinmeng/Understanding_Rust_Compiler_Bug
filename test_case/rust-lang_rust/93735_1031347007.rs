plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: associated function has missing const stability attribute
    --> library/core/src/num/int_macros.rs:2433:9
     |
1    |   / macro_rules! int_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |   |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
2433 | / |         pub const fn abs_diff(self, other: Self) -> $UnsignedT {
2434 | | |             if self < other {
2435 | | |                 // Converting a non-negative x from signed to unsigned by using
2436 | | |                 // `x as U` is left unchanged, but a negative x is converted
2451 | | |             }
2452 | | |         }
     | |_|_________^
...      |
...      |
2720 |   |     }
2721 |   | }
     |   |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:198:5
     |
198  | /       int_impl! { i8, i8, u8, 8, 7, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
199  | |       "[0x12]", "[0x12]", "", "" }

error: associated function has missing const stability attribute
    --> library/core/src/num/int_macros.rs:2433:9
     |
     |
1    |   / macro_rules! int_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |   |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
2433 | / |         pub const fn abs_diff(self, other: Self) -> $UnsignedT {
2434 | | |             if self < other {
2435 | | |                 // Converting a non-negative x from signed to unsigned by using
2436 | | |                 // `x as U` is left unchanged, but a negative x is converted
2451 | | |             }
2452 | | |         }
     | |_|_________^
...      |
...      |
2720 |   |     }
2721 |   | }
     |   |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:204:5
     |
204  | /       int_impl! { i16, i16, u16, 16, 15, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
205  | |       "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }

error: associated function has missing const stability attribute
    --> library/core/src/num/int_macros.rs:2433:9
     |
     |
1    |   / macro_rules! int_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |   |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
2433 | / |         pub const fn abs_diff(self, other: Self) -> $UnsignedT {
2434 | | |             if self < other {
2435 | | |                 // Converting a non-negative x from signed to unsigned by using
2436 | | |                 // `x as U` is left unchanged, but a negative x is converted
2451 | | |             }
2452 | | |         }
     | |_|_________^
...      |
...      |
2720 |   |     }
2721 |   | }
     |   |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:210:5
     |
210  | /       int_impl! { i32, i32, u32, 32, 31, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
211  | |       "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
212  | |       "[0x12, 0x34, 0x56, 0x78]", "", "" }

error: associated function has missing const stability attribute
    --> library/core/src/num/int_macros.rs:2433:9
     |
     |
1    |   / macro_rules! int_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |   |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
2433 | / |         pub const fn abs_diff(self, other: Self) -> $UnsignedT {
2434 | | |             if self < other {
2435 | | |                 // Converting a non-negative x from signed to unsigned by using
2436 | | |                 // `x as U` is left unchanged, but a negative x is converted
2451 | | |             }
2452 | | |         }
     | |_|_________^
...      |
...      |
2720 |   |     }
2721 |   | }
     |   |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:217:5
     |
217  | /       int_impl! { i64, i64, u64, 64, 63, -9223372036854775808, 9223372036854775807, 12,
218  | |       "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
219  | |       "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
220  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }

error: associated function has missing const stability attribute
    --> library/core/src/num/int_macros.rs:2433:9
     |
     |
1    |   / macro_rules! int_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |   |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
2433 | / |         pub const fn abs_diff(self, other: Self) -> $UnsignedT {
2434 | | |             if self < other {
2435 | | |                 // Converting a non-negative x from signed to unsigned by using
2436 | | |                 // `x as U` is left unchanged, but a negative x is converted
2451 | | |             }
2452 | | |         }
     | |_|_________^
...      |
...      |
2720 |   |     }
2721 |   | }
     |   |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:225:5
     |
225  | /       int_impl! { i128, i128, u128, 128, 127, -170141183460469231731687303715884105728,
226  | |       170141183460469231731687303715884105727, 16,
227  | |       "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
228  | |       "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
231  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
232  | |         0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }

error: associated function has missing const stability attribute
    --> library/core/src/num/int_macros.rs:2433:9
     |
     |
1    |   / macro_rules! int_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |   |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |   |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...      |
2433 | / |         pub const fn abs_diff(self, other: Self) -> $UnsignedT {
2434 | | |             if self < other {
2435 | | |                 // Converting a non-negative x from signed to unsigned by using
2436 | | |                 // `x as U` is left unchanged, but a negative x is converted
2451 | | |             }
2452 | | |         }
     | |_|_________^
...      |
...      |
2720 |   |     }
2721 |   | }
     |   |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:255:5
     |
255  | /       int_impl! { isize, i64, usize, 64, 63, -9223372036854775808, 9223372036854775807,
256  | |       12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
257  | |       "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
258  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
259  | |       usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }

error: associated function has missing const stability attribute
    --> library/core/src/num/uint_macros.rs:1645:9
     |
     |
1    |   / macro_rules! uint_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $NonZeroT:ident,
3    |   |         $BITS:expr, $MaxV:expr,
4    |   |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
...      |
1645 | / |         pub const fn abs_diff(self, other: Self) -> Self {
1646 | | |             if mem::size_of::<Self>() == 1 {
1647 | | |                 // Trick LLVM into generating the psadbw instruction when SSE2
1648 | | |                 // is available and this function is autovectorized for u8's.
1656 | | |             }
1657 | | |         }
     | |_|_________^
...      |
...      |
2436 |   |     }
2437 |   | }
     |   |_- in this expansion of `uint_impl!`
     |
    ::: library/core/src/num/mod.rs:267:5
     |
267  | /       uint_impl! { u8, u8, i8, NonZeroU8, 8, 255, 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
268  | |       "[0x12]", "", "" }

error: associated function has missing const stability attribute
    --> library/core/src/num/uint_macros.rs:1645:9
     |
     |
1    |   / macro_rules! uint_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $NonZeroT:ident,
3    |   |         $BITS:expr, $MaxV:expr,
4    |   |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
...      |
1645 | / |         pub const fn abs_diff(self, other: Self) -> Self {
1646 | | |             if mem::size_of::<Self>() == 1 {
1647 | | |                 // Trick LLVM into generating the psadbw instruction when SSE2
1648 | | |                 // is available and this function is autovectorized for u8's.
1656 | | |             }
1657 | | |         }
     | |_|_________^
...      |
...      |
2436 |   |     }
2437 |   | }
     |   |_- in this expansion of `uint_impl!`
     |
    ::: library/core/src/num/mod.rs:816:5
     |
816  | /       uint_impl! { u16, u16, i16, NonZeroU16, 16, 65535, 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
817  | |       "[0x34, 0x12]", "[0x12, 0x34]", "", "" }

error: associated function has missing const stability attribute
    --> library/core/src/num/uint_macros.rs:1645:9
     |
     |
1    |   / macro_rules! uint_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $NonZeroT:ident,
3    |   |         $BITS:expr, $MaxV:expr,
4    |   |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
...      |
1645 | / |         pub const fn abs_diff(self, other: Self) -> Self {
1646 | | |             if mem::size_of::<Self>() == 1 {
1647 | | |                 // Trick LLVM into generating the psadbw instruction when SSE2
1648 | | |                 // is available and this function is autovectorized for u8's.
1656 | | |             }
1657 | | |         }
     | |_|_________^
...      |
...      |
2436 |   |     }
2437 |   | }
     |   |_- in this expansion of `uint_impl!`
     |
    ::: library/core/src/num/mod.rs:823:5
     |
823  | /       uint_impl! { u32, u32, i32, NonZeroU32, 32, 4294967295, 8, "0x10000b3", "0xb301", "0x12345678",
824  | |       "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }

error: associated function has missing const stability attribute
    --> library/core/src/num/uint_macros.rs:1645:9
     |
     |
1    |   / macro_rules! uint_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $NonZeroT:ident,
3    |   |         $BITS:expr, $MaxV:expr,
4    |   |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
...      |
1645 | / |         pub const fn abs_diff(self, other: Self) -> Self {
1646 | | |             if mem::size_of::<Self>() == 1 {
1647 | | |                 // Trick LLVM into generating the psadbw instruction when SSE2
1648 | | |                 // is available and this function is autovectorized for u8's.
1656 | | |             }
1657 | | |         }
     | |_|_________^
...      |
...      |
2436 |   |     }
2437 |   | }
     |   |_- in this expansion of `uint_impl!`
     |
    ::: library/core/src/num/mod.rs:830:5
     |
830  | /       uint_impl! { u64, u64, i64, NonZeroU64, 64, 18446744073709551615, 12, "0xaa00000000006e1", "0x6e10aa",
831  | |       "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
832  | |       "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
833  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
834  | |       "", ""}

error: associated function has missing const stability attribute
    --> library/core/src/num/uint_macros.rs:1645:9
     |
     |
1    |   / macro_rules! uint_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $NonZeroT:ident,
3    |   |         $BITS:expr, $MaxV:expr,
4    |   |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
...      |
1645 | / |         pub const fn abs_diff(self, other: Self) -> Self {
1646 | | |             if mem::size_of::<Self>() == 1 {
1647 | | |                 // Trick LLVM into generating the psadbw instruction when SSE2
1648 | | |                 // is available and this function is autovectorized for u8's.
1656 | | |             }
1657 | | |         }
     | |_|_________^
...      |
...      |
2436 |   |     }
2437 |   | }
     |   |_- in this expansion of `uint_impl!`
     |
    ::: library/core/src/num/mod.rs:840:5
     |
840  | /       uint_impl! { u128, u128, i128, NonZeroU128, 128, 340282366920938463463374607431768211455, 16,
841  | |       "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
842  | |       "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
843  | |       "[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, \
846  | |         0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]",
847  | |        "", ""}
     | |______________- in this macro invocation


error: associated function has missing const stability attribute
    --> library/core/src/num/uint_macros.rs:1645:9
     |
1    |   / macro_rules! uint_impl {
2    |   |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $NonZeroT:ident,
3    |   |         $BITS:expr, $MaxV:expr,
4    |   |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
...      |
1645 | / |         pub const fn abs_diff(self, other: Self) -> Self {
1646 | | |             if mem::size_of::<Self>() == 1 {
1647 | | |                 // Trick LLVM into generating the psadbw instruction when SSE2
1648 | | |                 // is available and this function is autovectorized for u8's.
1656 | | |             }
1657 | | |         }
     | |_|_________^
...      |
...      |
2436 |   |     }
2437 |   | }
     |   |_- in this expansion of `uint_impl!`
     |
    ::: library/core/src/num/mod.rs:870:5
     |
870  | /       uint_impl! { usize, u64, isize, NonZeroUsize, 64, 18446744073709551615, 12, "0xaa00000000006e1", "0x6e10aa",
871  | |       "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
872  | |       "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
873  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
874  | |       usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }

error: could not compile `core` due to 12 previous errors
Build completed unsuccessfully in 0:04:30
