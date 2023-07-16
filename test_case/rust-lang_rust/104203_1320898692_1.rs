
error[E0080]: it is undefined behavior to use this value
 --> src/lib.rs:1:1
  |
1 | const U16_SLICE: &[u16] = {
  | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
  |
  = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
  = note: the raw bytes of the constant (size: 16, align: 8) {
              ╾───────alloc3────────╼ 02 00 00 00 00 00 00 00 │ ╾──────╼........
          }
