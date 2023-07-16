
 Compiling playground v0.0.1 (/playground)
error: implementation of `A` is not general enough
  --> src/main.rs:34:5
   |
34 |     wrap(do_something);
   |     ^^^^ implementation of `A` is not general enough
   |
   = note: `A<'0>` would have to be implemented for the type `&i32`, for any lifetime `'0`...
   = note: ...but `A<'1>` is actually implemented for the type `&'1 i32`, for some specific lifetime `'1`

error: could not compile `playground` due to previous error
