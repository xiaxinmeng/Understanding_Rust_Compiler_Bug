
error: unexpected token: `::`
 --> test.rs:3:38
  |
3 |     if i as usize < std::mem::size_of::<i64>() {
  |                                      ^^
  |
  = help: use `<...>` instead of `::<...>` if you meant to specify type arguments

error: expected one of `!`, `+`, `,`, `::`, or `>`, found `(`
 --> test.rs:3:45
  |
3 |     if i as usize < std::mem::size_of::<i64>() {
  |                                             ^ expected one of `!`, `+`, `,`, `::`, or `>` here

error: aborting due to 2 previous errors
