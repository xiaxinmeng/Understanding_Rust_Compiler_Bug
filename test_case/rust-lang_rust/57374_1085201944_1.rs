
error: implementation of `Fn` is not general enough
 --> src/lib.rs:7:27
  |
7 |   fn foo() -> impl Fn(&str) {
  |  ___________________________^
8 | |     bar()
9 | | }
  | |_^ implementation of `Fn` is not general enough
  |
  = note: `impl Fn(&'2 str)` must implement `Fn<(&'1 str,)>`, for any lifetime `'1`...
  = note: ...but it actually implements `Fn<(&'2 str,)>`, for some specific lifetime `'2`
