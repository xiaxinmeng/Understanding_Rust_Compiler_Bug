
   Compiling playground v0.0.1 (/playground)
error[E0369]: binary operation `==` cannot be applied to type `Corgi`
 --> src/lib.rs:7:10
  |
7 |   if lol == Corgi::Foo {
  |      --- ^^ ---------- Corgi
  |      |
  |      Corgi
  |
  = note: an implementation of `std::cmp::PartialEq` might be missing for `Corgi`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0369`.
error: could not compile `playground`.

To learn more, run the command again with --verbose.
