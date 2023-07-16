console
error: invalid `struct` delimiters or `fn` call arguments
 --> src/lib.rs:4:5
  |
4 |     Foo (0: x.0 as _)
  |     ^^^^^^^^^^^^^^^^^
  |
help: if `Foo` is a struct, use braces as delimiters
  |
4 |     Foo  { 0: x.0 as _ }
  |          ~             ~
help: if `Foo` is a function, use the arguments directly
  |
4 -     Foo (0: x.0 as _)
4 +     Foo (x.0 as _)
  |
