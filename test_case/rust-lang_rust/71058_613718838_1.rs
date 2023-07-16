
   Compiling playground v0.0.1 (/playground)
error: implementation of `MyUnpin` is not general enough
  --> src/lib.rs:32:5
   |
3  | pub auto trait MyUnpin {}
   | ------------------------- trait `MyUnpin` defined here
...
32 |     Box::new(async { x.await })
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `MyUnpin` is not general enough
   |
   = note: `MyUnpin` would have to be implemented for the type `&'0 &'1 ()`, for any two lifetimes `'0` and `'1`...
   = note: ...but `MyUnpin` is actually implemented for the type `&'2 &()`, for some specific lifetime `'2`

error: aborting due to previous error

error: could not compile `playground`.

To learn more, run the command again with --verbose.

