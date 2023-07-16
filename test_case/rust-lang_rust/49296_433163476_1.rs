rust
error[E0080]: this constant likely exhibits undefined behavior
  --> src/main.rs:16:1
   |
16 | const X: u64 = wat(42);
   | ^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain bits
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior

error: aborting due to previous error

