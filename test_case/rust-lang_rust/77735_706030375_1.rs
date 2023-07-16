bash
error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
 --> src/lib.rs:7:25
  |
7 | const Y0: i8 = unsafe { simd_extract(f32x3, 0) };
  |                         ^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0015`.
error: could not compile `playground`

To learn more, run the command again with --verbose.
