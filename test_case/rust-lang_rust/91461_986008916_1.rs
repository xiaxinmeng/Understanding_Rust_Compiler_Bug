
error: invalid `struct` delimiters or `fn` call arguments
 --> errors.rs:2:5
  |
2 |     a(_:b:,)
  |     ^^^^^^^^
  |
help: if `a` is a struct, use braces as delimiters
  |
2 |     a { _:b:, }
  |       ~       ~

