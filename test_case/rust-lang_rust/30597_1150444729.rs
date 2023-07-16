
error: expected expression, found `42;`
 --> src/main.rs:2:20
  |
2 |     ($s:stmt) => { $s }
  |                    ^^ expected expression
...
6 |     println!("{}", foo!(42));
  |                    -------- in this macro invocation
  |
  = note: the macro call doesn't expand to an expression, but it can expand to a statement
  = note: this error originates in the macro `foo` (in Nightly builds, run with -Z macro-backtrace for more info)
help: add `;` to interpret the expansion as a statement
  |
6 |     println!("{}", foo!(42););
  |                            +
