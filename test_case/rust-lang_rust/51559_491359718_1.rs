
   Compiling playground v0.0.1 (/playground)
error[E0080]: it is undefined behavior to use this value
 --> src/main.rs:4:1
  |
4 | pub const FOO: usize = unsafe { BAR as usize };
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes
  |
  = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
error: Could not compile `playground`.

To learn more, run the command again with --verbose.
