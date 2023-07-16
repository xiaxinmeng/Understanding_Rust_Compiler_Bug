plain
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: `num::<impl u8>::carrying_add` is not yet stable as a const fn
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1260 |  |             let (sum, carry) = (self as $UnsignedT).carrying_add(rhs as $UnsignedT, carry);
...     |
2059 |  |     }
2060 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:165:5
     |
165  | /      int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
166  | |      "[0x12]", "[0x12]", "", "" }
     |
     |
     = help: add `#![feature(const_carrying)]` to the crate attributes to enable

error: `num::<impl u16>::carrying_add` is not yet stable as a const fn
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1260 |  |             let (sum, carry) = (self as $UnsignedT).carrying_add(rhs as $UnsignedT, carry);
...     |
2059 |  |     }
2060 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:172:5
     |
172  | /      int_impl! { i16, i16, u16, 16, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
173  | |      "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     |
     |
     = help: add `#![feature(const_carrying)]` to the crate attributes to enable

error: `num::<impl u32>::carrying_add` is not yet stable as a const fn
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1260 |  |             let (sum, carry) = (self as $UnsignedT).carrying_add(rhs as $UnsignedT, carry);
...     |
2059 |  |     }
2060 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:179:5
     |
179  | /      int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
180  | |      "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
181  | |      "[0x12, 0x34, 0x56, 0x78]", "", "" }
     |
     |
     = help: add `#![feature(const_carrying)]` to the crate attributes to enable

error: `num::<impl u64>::carrying_add` is not yet stable as a const fn
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1260 |  |             let (sum, carry) = (self as $UnsignedT).carrying_add(rhs as $UnsignedT, carry);
...     |
2059 |  |     }
2060 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:187:5
     |
187  | /      int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, 12,
188  | |      "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
189  | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
190  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
     |
     |
     = help: add `#![feature(const_carrying)]` to the crate attributes to enable

error: `num::<impl u128>::carrying_add` is not yet stable as a const fn
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1260 |  |             let (sum, carry) = (self as $UnsignedT).carrying_add(rhs as $UnsignedT, carry);
...     |
2059 |  |     }
2060 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:195:5
     |
195  | /      int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
196  | |      170141183460469231731687303715884105727, 16,
197  | |      "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
198  | |      "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
201  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
202  | |        0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
     |
     |
     = help: add `#![feature(const_carrying)]` to the crate attributes to enable

error: `num::<impl usize>::carrying_add` is not yet stable as a const fn
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1260 |  |             let (sum, carry) = (self as $UnsignedT).carrying_add(rhs as $UnsignedT, carry);
...     |
2059 |  |     }
2060 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:228:5
     |
228  | /      int_impl! { isize, i64, usize, 64, -9223372036854775808, 9223372036854775807,
229  | |      12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
230  | |       "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
231  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
232  | |       usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
     |
     |
     = help: add `#![feature(const_carrying)]` to the crate attributes to enable

error: `num::<impl u8>::borrowing_sub` is not yet stable as a const fn
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1311 |  |             let (sum, carry) = (self as $UnsignedT).borrowing_sub(rhs as $UnsignedT, carry);
...     |
2059 |  |     }
2060 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:165:5
     |
165  | /      int_impl! { i8, i8, u8, 8, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
166  | |      "[0x12]", "[0x12]", "", "" }
     |
     |
     = help: add `#![feature(const_carrying)]` to the crate attributes to enable

error: `num::<impl u16>::borrowing_sub` is not yet stable as a const fn
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1311 |  |             let (sum, carry) = (self as $UnsignedT).borrowing_sub(rhs as $UnsignedT, carry);
...     |
2059 |  |     }
2060 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:172:5
     |
172  | /      int_impl! { i16, i16, u16, 16, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
173  | |      "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     |
     |
     = help: add `#![feature(const_carrying)]` to the crate attributes to enable

error: `num::<impl u32>::borrowing_sub` is not yet stable as a const fn
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1311 |  |             let (sum, carry) = (self as $UnsignedT).borrowing_sub(rhs as $UnsignedT, carry);
...     |
2059 |  |     }
2060 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:179:5
     |
179  | /      int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
180  | |      "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
181  | |      "[0x12, 0x34, 0x56, 0x78]", "", "" }
     |
     |
     = help: add `#![feature(const_carrying)]` to the crate attributes to enable

error: `num::<impl u64>::borrowing_sub` is not yet stable as a const fn
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1311 |  |             let (sum, carry) = (self as $UnsignedT).borrowing_sub(rhs as $UnsignedT, carry);
...     |
2059 |  |     }
2060 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:187:5
     |
187  | /      int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, 12,
188  | |      "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
189  | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
190  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
     |
     |
     = help: add `#![feature(const_carrying)]` to the crate attributes to enable

error: `num::<impl u128>::borrowing_sub` is not yet stable as a const fn
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1311 |  |             let (sum, carry) = (self as $UnsignedT).borrowing_sub(rhs as $UnsignedT, carry);
...     |
2059 |  |     }
2060 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:195:5
     |
195  | /      int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
196  | |      170141183460469231731687303715884105727, 16,
197  | |      "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
198  | |      "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
...    |
201  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
202  | |        0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
     |
     |
     = help: add `#![feature(const_carrying)]` to the crate attributes to enable

error: `num::<impl usize>::borrowing_sub` is not yet stable as a const fn
     |
1    |  / macro_rules! int_impl {
1    |  / macro_rules! int_impl {
2    |  |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr,
3    |  |      $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    |  |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...     |
1311 |  |             let (sum, carry) = (self as $UnsignedT).borrowing_sub(rhs as $UnsignedT, carry);
...     |
2059 |  |     }
2060 |  | }
     |  |_- in this expansion of `int_impl!`
     |  |_- in this expansion of `int_impl!`
     | 
    ::: library/core/src/num/mod.rs:228:5
     |
228  | /      int_impl! { isize, i64, usize, 64, -9223372036854775808, 9223372036854775807,
229  | |      12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
230  | |       "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
231  | |       "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
232  | |       usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
     |
     |
     = help: add `#![feature(const_carrying)]` to the crate attributes to enable
error: aborting due to 12 previous errors

error: could not compile `core`

