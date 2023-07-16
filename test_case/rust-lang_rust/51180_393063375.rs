
$ cargo build
   Compiling faster v0.4.3
error[E0463]: can't find crate for `core_or_std`
  --> src/intrin/eq.rs:58:25
   |
58 |                       use core_or_std::mem::transmute;
   |                           ^^^^^^^^^^^ can't find crate
...
79 | / rust_fallback_eq! {
80 | |     impl Eq for u8x16 where "sse2" {
81 | |         eq_mask, eq => u8x16, u8, _mm_cmpeq_epi8(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
82 | |     }
83 | | }
   | |_- in this macro invocation

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: Could not compile `faster`.

To learn more, run the command again with --verbose.
