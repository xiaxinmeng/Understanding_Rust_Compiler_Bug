text
; rustdoc +nightly-2022-10-11 t.rs
error[E0516]: `typeof` is a reserved keyword but unimplemented
 --> t.rs:2:9
  |
2 |     y: (typeof("hey"),),
  |         ^^^^^^^^^^^^^ reserved keyword
  |
help: consider replacing `typeof(...)` with an actual type
  |
2 |     y: (&'static str,),
  |         ~~~~~~~~~~~~

error: aborting due to previous error

For more information about this error, try `rustc --explain E0516`.
