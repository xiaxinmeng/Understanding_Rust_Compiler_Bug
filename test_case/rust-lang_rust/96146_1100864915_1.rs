
error: implementation of `Trait` is not general enough
 --> src/lib.rs:6:5
  |
6 |     0_i32
  |     ^^^^^ implementation of `Trait` is not general enough
  |
  = note: `i32` must implement `Trait<'0>`, for any lifetime `'0`...
  = note: ...but it actually implements `Trait<'1>`, for some specific lifetime `'1`
