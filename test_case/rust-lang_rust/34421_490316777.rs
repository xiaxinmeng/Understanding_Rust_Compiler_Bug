
error: expected expression, found keyword `struct`
 --> src/lib.rs:2:21
  |
2 |     ($a:ident) => { struct $a; }
  |                     ^^^^^^ expected expression
...
5 | fn a() { make_item!(A) }
  |          ------------- in this macro invocation
