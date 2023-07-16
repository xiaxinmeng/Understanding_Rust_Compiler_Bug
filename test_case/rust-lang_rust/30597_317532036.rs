
error: expected expression, found `42;`
 --> test.rs:2:20
  |
2 |     ($s:stmt) => { $s }
  |                    ^^ expected expression
...
6 |     println!("{}", foo!(42));
  |                    -------- in this macro invocation
