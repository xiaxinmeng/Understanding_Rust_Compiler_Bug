plain
   --> src/constant.rs:175:33
    |
175 |                 let mut alloc = Allocation::from_bytes(
    |                                 ^^^^^^^^^^^^^^^^^^^^^^ expected 3 arguments
176 |                     std::iter::repeat(0).take(size.bytes_usize()).collect::<Vec<u8>>(),
177 |                     align,
    |                     ----- supplied 2 arguments
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/mir/interpret/allocation.rs:103:12
    |
103 |     pub fn from_bytes<'a>(slice: impl Into<Cow<'a, [u8]>>, align: Align, mutability: Mutability) -> Self {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0061`.
