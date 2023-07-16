
error[E0004]: non-exhaustive patterns: `true` not covered
 --> src/lib.rs:2:11
  |
2 |     match a {
  |           ^ pattern `true` not covered
  |
  = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
  = note: the matched value is of type `bool`
