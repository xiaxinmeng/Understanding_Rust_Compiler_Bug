plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.85
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i8>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2635 | |          pub const fn to_be_bytes(self) -> [u8; mem::size_of::<Self>()] {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:241:5
     |
241  |  /     int_impl! { i8, i8, u8, 8, 7, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
242  |  |     "[0x12]", "[0x12]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i8>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i8>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2655 | |          pub const fn to_le_bytes(self) -> [u8; mem::size_of::<Self>()] {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:241:5
     |
241  |  /     int_impl! { i8, i8, u8, 8, 7, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
242  |  |     "[0x12]", "[0x12]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i8>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i8>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2691 | |          pub const fn to_ne_bytes(self) -> [u8; mem::size_of::<Self>()] {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:241:5
     |
241  |  /     int_impl! { i8, i8, u8, 8, 7, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
242  |  |     "[0x12]", "[0x12]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i8>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i8>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2722 | |          pub const fn from_be_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:241:5
     |
241  |  /     int_impl! { i8, i8, u8, 8, 7, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
242  |  |     "[0x12]", "[0x12]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i8>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i8>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2751 | |          pub const fn from_le_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:241:5
     |
241  |  /     int_impl! { i8, i8, u8, 8, 7, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
242  |  |     "[0x12]", "[0x12]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i8>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i8>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2793 | |          pub const fn from_ne_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:241:5
     |
241  |  /     int_impl! { i8, i8, u8, 8, 7, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
242  |  |     "[0x12]", "[0x12]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i8>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i16>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2635 | |          pub const fn to_be_bytes(self) -> [u8; mem::size_of::<Self>()] {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:246:5
     |
246  |  /     int_impl! { i16, i16, u16, 16, 15, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
247  |  |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i16>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i16>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2655 | |          pub const fn to_le_bytes(self) -> [u8; mem::size_of::<Self>()] {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:246:5
     |
246  |  /     int_impl! { i16, i16, u16, 16, 15, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
247  |  |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i16>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i16>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2691 | |          pub const fn to_ne_bytes(self) -> [u8; mem::size_of::<Self>()] {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:246:5
     |
246  |  /     int_impl! { i16, i16, u16, 16, 15, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
247  |  |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i16>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i16>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2722 | |          pub const fn from_be_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:246:5
     |
246  |  /     int_impl! { i16, i16, u16, 16, 15, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
247  |  |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i16>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i16>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2751 | |          pub const fn from_le_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:246:5
     |
246  |  /     int_impl! { i16, i16, u16, 16, 15, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
247  |  |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i16>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i16>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2793 | |          pub const fn from_ne_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:246:5
     |
246  |  /     int_impl! { i16, i16, u16, 16, 15, -32768, 32767, 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
247  |  |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i16>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i32>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2635 | |          pub const fn to_be_bytes(self) -> [u8; mem::size_of::<Self>()] {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:251:5
     |
251  |  /     int_impl! { i32, i32, u32, 32, 31, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
252  |  |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
253  |  |     "[0x12, 0x34, 0x56, 0x78]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i32>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i32>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2655 | |          pub const fn to_le_bytes(self) -> [u8; mem::size_of::<Self>()] {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:251:5
     |
251  |  /     int_impl! { i32, i32, u32, 32, 31, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
252  |  |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
253  |  |     "[0x12, 0x34, 0x56, 0x78]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i32>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i32>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2691 | |          pub const fn to_ne_bytes(self) -> [u8; mem::size_of::<Self>()] {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:251:5
     |
251  |  /     int_impl! { i32, i32, u32, 32, 31, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
252  |  |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
253  |  |     "[0x12, 0x34, 0x56, 0x78]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i32>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i32>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2722 | |          pub const fn from_be_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:251:5
     |
251  |  /     int_impl! { i32, i32, u32, 32, 31, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
252  |  |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
253  |  |     "[0x12, 0x34, 0x56, 0x78]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i32>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i32>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2751 | |          pub const fn from_le_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:251:5
     |
251  |  /     int_impl! { i32, i32, u32, 32, 31, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
252  |  |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
253  |  |     "[0x12, 0x34, 0x56, 0x78]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i32>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i32>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2793 | |          pub const fn from_ne_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:251:5
     |
251  |  /     int_impl! { i32, i32, u32, 32, 31, -2147483648, 2147483647, 8, "0x10000b3", "0xb301",
252  |  |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
253  |  |     "[0x12, 0x34, 0x56, 0x78]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i32>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i64>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2635 | |          pub const fn to_be_bytes(self) -> [u8; mem::size_of::<Self>()] {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:257:5
     |
257  |  /     int_impl! { i64, i64, u64, 64, 63, -9223372036854775808, 9223372036854775807, 12,
258  |  |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
259  |  |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
260  |  |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i64>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i64>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2655 | |          pub const fn to_le_bytes(self) -> [u8; mem::size_of::<Self>()] {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:257:5
     |
257  |  /     int_impl! { i64, i64, u64, 64, 63, -9223372036854775808, 9223372036854775807, 12,
258  |  |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
259  |  |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
260  |  |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i64>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i64>}: Callable<()>`
     |
1    | /  macro_rules! int_impl {
1    | /  macro_rules! int_impl {
2    | |      ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $BITS_MINUS_ONE:expr, $Min:expr, $Max:expr,
3    | |       $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
4    | |       $reversed:expr, $le_bytes:expr, $be_bytes:expr,
...    |
2691 | |          pub const fn to_ne_bytes(self) -> [u8; mem::size_of::<Self>()] {
...    |
2823 | |      }
2824 | |  }
     | |__- in this expansion of `int_impl!`
     | |__- in this expansion of `int_impl!`
     |
    ::: library/core/src/num/mod.rs:257:5
     |
257  |  /     int_impl! { i64, i64, u64, 64, 63, -9223372036854775808, 9223372036854775807, 12,
258  |  |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
259  |  |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
260  |  |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "", "" }
     |
     |
     = note: cannot satisfy `fn() -> usize {size_of::<i64>}: Callable<()>`

error[E0283]: type annotations needed: cannot satisfy `fn() -> usize {size_of::<i64>}: Callable<()>`
