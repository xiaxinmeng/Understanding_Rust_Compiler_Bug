
error[E0004]: non-exhaustive patterns: `&[_, ..]` not covered
 --> src/main.rs:2:11
  |
2 |     match data {
  |           ^^^^ pattern `&[_, ..]` not covered
  |
  = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
  = note: the matched value is of type `&[u8]`
