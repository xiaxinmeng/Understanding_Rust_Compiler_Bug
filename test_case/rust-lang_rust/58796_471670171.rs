
error: unexpected end of macro invocation
 --> file.rs:6:24
  |
1 | macro_rules! log {
  | ---------------- when calling this macro
...
6 |     log!(Level::Error ,);
  |                        ^ missing tokens in macro arguments
