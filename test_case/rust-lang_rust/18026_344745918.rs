
error[E0309]: the parameter type `CL` may not live long enough
 --> src/main.rs:8:5
  |
5 | pub struct Iter<'a, 'b, T: FromRedisValue, CL: ConnectionLike> {
  |                                            --- help: consider adding an explicit lifetime bound `CL: 'b`...
...
8 |     con: &'b CL,
  |     ^^^^^^^^^^^
  |
note: ...so that the reference type `&'b CL` does not outlive the data it points at
 --> src/main.rs:8:5
  |
8 |     con: &'b CL,
  |     ^^^^^^^^^^^
