
error[E0080]: constant evaluation error
 --> src/main.rs:1:25
  |
1 | const CONST_REF: &i64 = &5;
  |                         ^^ unimplemented constant expression: address operator
  |
note: for pattern here
 --> src/main.rs:7:9
  |
7 |         CONST_REF => (),
  |         ^^^^^^^^^
