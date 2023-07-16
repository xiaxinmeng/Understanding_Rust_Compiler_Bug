
   Compiling playground v0.0.1 (file:///playground)
error: can't qualify macro invocation with `pub`
 --> src/main.rs:1:1
  |
1 | pub get(x: u8) {}
  | ^^^
  |
  = help: try adjusting the macro to put `pub` inside the invocation

error: expected one of `!` or `::`, found `(`
 --> src/main.rs:1:8
  |
1 | pub get(x: u8) {}
  |        ^ expected one of `!` or `::` here

error: aborting due to 2 previous errors

error: Could not compile `playground`.
