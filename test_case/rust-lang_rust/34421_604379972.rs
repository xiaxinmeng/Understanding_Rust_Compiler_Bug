

error: expected expression, found keyword `struct`
 --> src/lib.rs:2:21
  |
2 |     ($a:ident) => { struct $a; }
  |                     ^^^^^^ expected expression
...
5 | fn a() { make_item!(A) }
  |          ------------- in this macro invocation
  |
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: expected expression, found keyword `struct`
 --> src/lib.rs:2:21
  |
2 |     ($a:ident) => { struct $a; }
  |                     ^^^^^^ expected expression
...
6 | fn b() { make_item!(B) }
  |          ------------- in this macro invocation
  |
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors
