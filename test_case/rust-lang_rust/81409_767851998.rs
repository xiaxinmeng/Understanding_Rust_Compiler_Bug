plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0308]: mismatched types
  --> library/core/src/str/iter.rs:50:53
   |
50 |         self.iter.filter(|&byte| !utf8_is_cont_byte(byte)).count()
   |                                                     |
   |                                                     |
   |                                                     expected `u8`, found `&u8`
   |                                                     help: consider dereferencing the borrow: `*byte`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `core`
