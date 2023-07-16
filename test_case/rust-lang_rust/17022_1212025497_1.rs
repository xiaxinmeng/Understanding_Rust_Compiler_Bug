
error: macro expansion ignores token `{` and any following
 --> src/lib.rs:2:12
  |
2 |       () => {{
  |  ____________^
3 | |         struct Foo;
4 | |     }}
  | |_____^
...
7 |   x!();
  |   ---- caused by the macro expansion here
  |
  = note: the usage of `x!` is likely invalid in item context
