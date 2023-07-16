
➜  hlua git:(rust-2018-migration) ✗ cargo +nightly build
   Compiling hlua v0.4.1 (file:///home/mkpankov/projects/hlua/hlua)
error[E0463]: can't find crate for `ffi`
 --> hlua/src/any.rs:1:5
  |
1 | use ffi;
  |     ^^^ can't find crate

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: Could not compile `hlua`.

To learn more, run the command again with --verbose.
