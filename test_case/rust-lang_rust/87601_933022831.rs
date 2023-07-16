plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
892  |  |             self.checked_add_unsigned(rhs).unwrap_or(Self::MAX)
...     |
2681 |  |     }
2682 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:172:5
     |
172  | /      int_impl! { i8, i8, u8, 8, 7, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
173  | |      "[0x12]", "[0x12]", "", "" }


error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
935  |  |             self.checked_sub_unsigned(rhs).unwrap_or(Self::MIN)
...     |
2681 |  |     }
2682 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:172:5
     |
172  | /      int_impl! { i8, i8, u8, 8, 7, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
173  | |      "[0x12]", "[0x12]", "", "" }


error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
892  |  |             self.checked_add_unsigned(rhs).unwrap_or(Self::MAX)
...     |
2681 |  |     }
2682 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:179:5
     |
179  | /      int_impl! { i16, i16, u16, 16, 15, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
180  | |      "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }


error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
935  |  |             self.checked_sub_unsigned(rhs).unwrap_or(Self::MIN)
...     |
2681 |  |     }
2682 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:179:5
     |
179  | /      int_impl! { i16, i16, u16, 16, 15, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
180  | |      "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }


error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
892  |  |             self.checked_add_unsigned(rhs).unwrap_or(Self::MAX)
...     |
2681 |  |     }
2682 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:186:5
     |
186  | /      int_impl! { i32, i32, u32, 32, 31, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
187  | |      "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
188  | |      "[0x12, 0x34, 0x56, 0x78]", "", "" }


error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
935  |  |             self.checked_sub_unsigned(rhs).unwrap_or(Self::MIN)
...     |
2681 |  |     }
2682 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:186:5
     |
186  | /      int_impl! { i32, i32, u32, 32, 31, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
187  | |      "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
188  | |      "[0x12, 0x34, 0x56, 0x78]", "", "" }


error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
892  |  |             self.checked_add_unsigned(rhs).unwrap_or(Self::MAX)
...     |
2681 |  |     }
2682 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:194:5
     |
194  | /      int_impl! { i64, i64, u64, 64, 63, -9223372036854775808, 9223372036854775807, 12,
195  | |      "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
196  | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
197  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }


error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
935  |  |             self.checked_sub_unsigned(rhs).unwrap_or(Self::MIN)
...     |
2681 |  |     }
2682 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:194:5
     |
194  | /      int_impl! { i64, i64, u64, 64, 63, -9223372036854775808, 9223372036854775807, 12,
195  | |      "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
196  | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
197  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }


error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
892  |  |             self.checked_add_unsigned(rhs).unwrap_or(Self::MAX)
...     |
2681 |  |     }
2682 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:202:5
     |
202  | /      int_impl! { i128, i128, u128, 128, 127, -170141183460469231731687303715884105728,
203  | |      170141183460469231731687303715884105727, 16,
204  | |      "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
205  | |      "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
208  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
209  | |        0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }


error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
935  |  |             self.checked_sub_unsigned(rhs).unwrap_or(Self::MIN)
...     |
2681 |  |     }
2682 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:202:5
     |
202  | /      int_impl! { i128, i128, u128, 128, 127, -170141183460469231731687303715884105728,
203  | |      170141183460469231731687303715884105727, 16,
204  | |      "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
205  | |      "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
208  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
209  | |        0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }


error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
892  |  |             self.checked_add_unsigned(rhs).unwrap_or(Self::MAX)
...     |
2681 |  |     }
2682 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:235:5
     |
235  | /      int_impl! { isize, i64, usize, 64, 63, -9223372036854775808, 9223372036854775807,
236  | |      12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
237  | |       "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
238  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
239  | |       usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }


error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
935  |  |             self.checked_sub_unsigned(rhs).unwrap_or(Self::MIN)
...     |
2681 |  |     }
2682 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:235:5
     |
235  | /      int_impl! { isize, i64, usize, 64, 63, -9223372036854775808, 9223372036854775807,
236  | |      12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
237  | |       "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
238  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
239  | |       usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }

For more information about this error, try `rustc --explain E0015`.
error: could not compile `core` due to 12 previous errors
Build completed unsuccessfully in 0:01:13
