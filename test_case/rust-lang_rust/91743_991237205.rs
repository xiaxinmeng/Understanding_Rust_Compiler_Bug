plain
 finished in 81.691 seconds
Set({"library/core"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling core v0.0.0 (/checkout/library/core)
error: this arithmetic operation will overflow
    --> library/core/tests/num/uint_macros.rs:15:25
     |
15   |                    assert!((MIN + MAX).wrapping_add(1) == 0);
     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `u128::MAX + 1_u128`, which would overflow
    ::: /checkout/library/core/src/num/uint_macros.rs:1:1
     |
1    |  / macro_rules! uint_impl {
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
1180 |  |             intrinsics::wrapping_add(self, rhs)
     |  |             ----------------------------------- in the inlined copy of this code (#2)
...     |
2439 |  |     }
2439 |  |     }
2440 |  | }
     |  |_- in this expansion of `uint_impl!` (#1)
    ::: /checkout/library/core/src/num/mod.rs:840:5
     |
     |
840  | /      uint_impl! { u128, u128, i128, 128, 340282366920938463463374607431768211455, 16,
841  | |      "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
842  | |      "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
843  | |      "[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, \
846  | |        0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]",
847  | |       "", ""}
     | |_____________- in this macro invocation (#1)
     |
     |
     = note: `#[deny(arithmetic_overflow)]` on by default

error: this arithmetic operation will overflow
    --> library/core/tests/num/uint_macros.rs:15:25
     |
15   |                    assert!((MIN + MAX).wrapping_add(1) == 0);
     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `u16::MAX + 1_u16`, which would overflow
    ::: /checkout/library/core/src/num/uint_macros.rs:1:1
     |
1    |  / macro_rules! uint_impl {
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
1180 |  |             intrinsics::wrapping_add(self, rhs)
     |  |             ----------------------------------- in the inlined copy of this code (#2)
...     |
2439 |  |     }
2439 |  |     }
2440 |  | }
     |  |_- in this expansion of `uint_impl!` (#1)
    ::: /checkout/library/core/src/num/mod.rs:816:5
     |
     |
816  | /      uint_impl! { u16, u16, i16, 16, 65535, 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
817  | |      "[0x34, 0x12]", "[0x12, 0x34]", "", "" }


error: this arithmetic operation will overflow
    --> library/core/tests/num/uint_macros.rs:15:25
     |
15   |                    assert!((MIN + MAX).wrapping_add(1) == 0);
     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `u32::MAX + 1_u32`, which would overflow
    ::: /checkout/library/core/src/num/uint_macros.rs:1:1
     |
1    |  / macro_rules! uint_impl {
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
1180 |  |             intrinsics::wrapping_add(self, rhs)
     |  |             ----------------------------------- in the inlined copy of this code (#2)
...     |
2439 |  |     }
2439 |  |     }
2440 |  | }
     |  |_- in this expansion of `uint_impl!` (#1)
    ::: /checkout/library/core/src/num/mod.rs:823:5
     |
     |
823  | /      uint_impl! { u32, u32, i32, 32, 4294967295, 8, "0x10000b3", "0xb301", "0x12345678",
824  | |      "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }


error: this arithmetic operation will overflow
    --> library/core/tests/num/uint_macros.rs:15:25
     |
15   |                    assert!((MIN + MAX).wrapping_add(1) == 0);
     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `u64::MAX + 1_u64`, which would overflow
    ::: /checkout/library/core/src/num/uint_macros.rs:1:1
     |
1    |  / macro_rules! uint_impl {
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
1180 |  |             intrinsics::wrapping_add(self, rhs)
     |  |             ----------------------------------- in the inlined copy of this code (#2)
...     |
2439 |  |     }
2439 |  |     }
2440 |  | }
     |  |_- in this expansion of `uint_impl!` (#1)
    ::: /checkout/library/core/src/num/mod.rs:830:5
     |
     |
830  | /      uint_impl! { u64, u64, i64, 64, 18446744073709551615, 12, "0xaa00000000006e1", "0x6e10aa",
831  | |      "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
832  | |      "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
833  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
834  | |      "", ""}


error: this arithmetic operation will overflow
    --> library/core/tests/num/uint_macros.rs:15:25
     |
15   |                    assert!((MIN + MAX).wrapping_add(1) == 0);
     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
    ::: /checkout/library/core/src/num/uint_macros.rs:1:1
     |
1    |  / macro_rules! uint_impl {
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
1180 |  |             intrinsics::wrapping_add(self, rhs)
     |  |             ----------------------------------- in the inlined copy of this code (#2)
...     |
2439 |  |     }
2439 |  |     }
2440 |  | }
     |  |_- in this expansion of `uint_impl!` (#1)
    ::: /checkout/library/core/src/num/mod.rs:267:5
     |
     |
267  | /      uint_impl! { u8, u8, i8, 8, 255, 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
268  | |      "[0x12]", "", "" }


error: this arithmetic operation will overflow
    --> library/core/tests/num/wrapping.rs:82:16
     |
82   |        assert_eq!(i8::MAX.wrapping_add(1), i8::MIN);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i8::MAX + 1_i8`, which would overflow
    ::: /checkout/library/core/src/num/int_macros.rs:1:1
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
1120 |  |             intrinsics::wrapping_add(self, rhs)
     |  |             ----------------------------------- in the inlined copy of this code (#2)
...     |
2722 |  |     }
2722 |  |     }
2723 |  | }
     |  |_- in this expansion of `int_impl!` (#1)
     |
    ::: /checkout/library/core/src/num/mod.rs:198:5
     |
198  | /      int_impl! { i8, i8, u8, 8, 7, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
199  | |      "[0x12]", "[0x12]", "", "" }


error: this arithmetic operation will overflow
    --> library/core/tests/num/wrapping.rs:83:16
     |
83   |        assert_eq!(i16::MAX.wrapping_add(1), i16::MIN);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i16::MAX + 1_i16`, which would overflow
    ::: /checkout/library/core/src/num/int_macros.rs:1:1
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
1120 |  |             intrinsics::wrapping_add(self, rhs)
     |  |             ----------------------------------- in the inlined copy of this code (#2)
...     |
2722 |  |     }
2722 |  |     }
2723 |  | }
     |  |_- in this expansion of `int_impl!` (#1)
     |
    ::: /checkout/library/core/src/num/mod.rs:204:5
     |
204  | /      int_impl! { i16, i16, u16, 16, 15, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
205  | |      "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }


error: this arithmetic operation will overflow
    --> library/core/tests/num/wrapping.rs:84:16
     |
84   |        assert_eq!(i32::MAX.wrapping_add(1), i32::MIN);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i32::MAX + 1_i32`, which would overflow
    ::: /checkout/library/core/src/num/int_macros.rs:1:1
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
1120 |  |             intrinsics::wrapping_add(self, rhs)
     |  |             ----------------------------------- in the inlined copy of this code (#2)
...     |
2722 |  |     }
2722 |  |     }
2723 |  | }
     |  |_- in this expansion of `int_impl!` (#1)
     |
    ::: /checkout/library/core/src/num/mod.rs:210:5
     |
210  | /      int_impl! { i32, i32, u32, 32, 31, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
211  | |      "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
212  | |      "[0x12, 0x34, 0x56, 0x78]", "", "" }


error: this arithmetic operation will overflow
    --> library/core/tests/num/wrapping.rs:85:16
     |
85   |        assert_eq!(i64::MAX.wrapping_add(1), i64::MIN);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i64::MAX + 1_i64`, which would overflow
    ::: /checkout/library/core/src/num/int_macros.rs:1:1
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
1120 |  |             intrinsics::wrapping_add(self, rhs)
     |  |             ----------------------------------- in the inlined copy of this code (#2)
...     |
2722 |  |     }
2722 |  |     }
2723 |  | }
     |  |_- in this expansion of `int_impl!` (#1)
     |
    ::: /checkout/library/core/src/num/mod.rs:217:5
     |
217  | /      int_impl! { i64, i64, u64, 64, 63, -9223372036854775808, 9223372036854775807, 12,
218  | |      "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
219  | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
220  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }


error: this arithmetic operation will overflow
    --> library/core/tests/num/wrapping.rs:86:16
     |
86   |        assert_eq!(isize::MAX.wrapping_add(1), isize::MIN);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `isize::MAX + 1_isize`, which would overflow
    ::: /checkout/library/core/src/num/int_macros.rs:1:1
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
1120 |  |             intrinsics::wrapping_add(self, rhs)
     |  |             ----------------------------------- in the inlined copy of this code (#2)
...     |
2722 |  |     }
2722 |  |     }
2723 |  | }
     |  |_- in this expansion of `int_impl!` (#1)
     |
    ::: /checkout/library/core/src/num/mod.rs:255:5
     |
255  | /      int_impl! { isize, i64, usize, 64, 63, -9223372036854775808, 9223372036854775807,
256  | |      12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
257  | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
258  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
259  | |      usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }


error: this arithmetic operation will overflow
    --> library/core/tests/num/wrapping.rs:88:16
     |
88   |        assert_eq!(i8::MIN.wrapping_sub(1), i8::MAX);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i8::MIN - 1_i8`, which would overflow
    ::: /checkout/library/core/src/num/int_macros.rs:1:1
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
1161 |  |             intrinsics::wrapping_sub(self, rhs)
     |  |             ----------------------------------- in the inlined copy of this code (#2)
...     |
2722 |  |     }
2722 |  |     }
2723 |  | }
     |  |_- in this expansion of `int_impl!` (#1)
     |
    ::: /checkout/library/core/src/num/mod.rs:198:5
     |
198  | /      int_impl! { i8, i8, u8, 8, 7, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
199  | |      "[0x12]", "[0x12]", "", "" }


error: this arithmetic operation will overflow
    --> library/core/tests/num/wrapping.rs:89:16
     |
89   |        assert_eq!(i16::MIN.wrapping_sub(1), i16::MAX);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i16::MIN - 1_i16`, which would overflow
    ::: /checkout/library/core/src/num/int_macros.rs:1:1
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
1161 |  |             intrinsics::wrapping_sub(self, rhs)
     |  |             ----------------------------------- in the inlined copy of this code (#2)
...     |
2722 |  |     }
2722 |  |     }
2723 |  | }
     |  |_- in this expansion of `int_impl!` (#1)
     |
    ::: /checkout/library/core/src/num/mod.rs:204:5
     |
204  | /      int_impl! { i16, i16, u16, 16, 15, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
205  | |      "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }


error: this arithmetic operation will overflow
    --> library/core/tests/num/wrapping.rs:90:16
     |
90   |        assert_eq!(i32::MIN.wrapping_sub(1), i32::MAX);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i32::MIN - 1_i32`, which would overflow
    ::: /checkout/library/core/src/num/int_macros.rs:1:1
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
1161 |  |             intrinsics::wrapping_sub(self, rhs)
     |  |             ----------------------------------- in the inlined copy of this code (#2)
...     |
2722 |  |     }
2722 |  |     }
2723 |  | }
     |  |_- in this expansion of `int_impl!` (#1)
     |
    ::: /checkout/library/core/src/num/mod.rs:210:5
     |
210  | /      int_impl! { i32, i32, u32, 32, 31, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
211  | |      "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
212  | |      "[0x12, 0x34, 0x56, 0x78]", "", "" }


error: this arithmetic operation will overflow
    --> library/core/tests/num/wrapping.rs:91:16
     |
91   |        assert_eq!(i64::MIN.wrapping_sub(1), i64::MAX);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i64::MIN - 1_i64`, which would overflow
    ::: /checkout/library/core/src/num/int_macros.rs:1:1
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
1161 |  |             intrinsics::wrapping_sub(self, rhs)
     |  |             ----------------------------------- in the inlined copy of this code (#2)
...     |
2722 |  |     }
2722 |  |     }
2723 |  | }
     |  |_- in this expansion of `int_impl!` (#1)
     |
    ::: /checkout/library/core/src/num/mod.rs:217:5
     |
217  | /      int_impl! { i64, i64, u64, 64, 63, -9223372036854775808, 9223372036854775807, 12,
218  | |      "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
219  | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
220  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }


error: this arithmetic operation will overflow
    --> library/core/tests/num/wrapping.rs:92:16
     |
92   |        assert_eq!(isize::MIN.wrapping_sub(1), isize::MAX);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `isize::MIN - 1_isize`, which would overflow
    ::: /checkout/library/core/src/num/int_macros.rs:1:1
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
1161 |  |             intrinsics::wrapping_sub(self, rhs)
     |  |             ----------------------------------- in the inlined copy of this code (#2)
...     |
2722 |  |     }
2722 |  |     }
2723 |  | }
     |  |_- in this expansion of `int_impl!` (#1)
     |
    ::: /checkout/library/core/src/num/mod.rs:255:5
     |
255  | /      int_impl! { isize, i64, usize, 64, 63, -9223372036854775808, 9223372036854775807,
256  | |      12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
257  | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
258  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
259  | |      usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }


error: this arithmetic operation will overflow
    --> library/core/tests/num/wrapping.rs:94:16
     |
94   |        assert_eq!(u8::MAX.wrapping_add(1), u8::MIN);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
    ::: /checkout/library/core/src/num/uint_macros.rs:1:1
     |
1    |  / macro_rules! uint_impl {
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
1180 |  |             intrinsics::wrapping_add(self, rhs)
     |  |             ----------------------------------- in the inlined copy of this code (#2)
...     |
2439 |  |     }
2439 |  |     }
2440 |  | }
     |  |_- in this expansion of `uint_impl!` (#1)
    ::: /checkout/library/core/src/num/mod.rs:267:5
     |
     |
267  | /      uint_impl! { u8, u8, i8, 8, 255, 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
268  | |      "[0x12]", "", "" }


error: this arithmetic operation will overflow
    --> library/core/tests/num/wrapping.rs:95:16
     |
95   |        assert_eq!(u16::MAX.wrapping_add(1), u16::MIN);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `u16::MAX + 1_u16`, which would overflow
    ::: /checkout/library/core/src/num/uint_macros.rs:1:1
     |
1    |  / macro_rules! uint_impl {
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
1180 |  |             intrinsics::wrapping_add(self, rhs)
     |  |             ----------------------------------- in the inlined copy of this code (#2)
...     |
2439 |  |     }
2439 |  |     }
2440 |  | }
     |  |_- in this expansion of `uint_impl!` (#1)
    ::: /checkout/library/core/src/num/mod.rs:816:5
     |
     |
816  | /      uint_impl! { u16, u16, i16, 16, 65535, 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
817  | |      "[0x34, 0x12]", "[0x12, 0x34]", "", "" }


error: this arithmetic operation will overflow
    --> library/core/tests/num/wrapping.rs:96:16
     |
96   |        assert_eq!(u32::MAX.wrapping_add(1), u32::MIN);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `u32::MAX + 1_u32`, which would overflow
    ::: /checkout/library/core/src/num/uint_macros.rs:1:1
     |
1    |  / macro_rules! uint_impl {
1    |  / macro_rules! uint_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $SignedT:ident, $BITS:expr, $MaxV:expr,
3    |  |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
1180 |  |             intrinsics::wrapping_add(self, rhs)
     |  |             ----------------------------------- in the inlined copy of this code (#2)
...     |
2439 |  |     }
2439 |  |     }
2440 |  | }
     |  |_- in this expansion of `uint_impl!` (#1)
    ::: /checkout/library/core/src/num/mod.rs:823:5
     |
     |
823  | /      uint_impl! { u32, u32, i32, 32, 4294967295, 8, "0x10000b3", "0xb301", "0x12345678",
824  | |      "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }


error: this arithmetic operation will overflow
    --> library/core/tests/num/wrapping.rs:97:16
     |
97   |        assert_eq!(u64::MAX.wrapping_add(1), u64::MIN);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `u64::MAX + 1_u64`, which would overflow
    ::: /checkout/library/core/src/num/uint_macros.rs:1:1
