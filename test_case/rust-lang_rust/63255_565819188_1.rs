
error: can't qualify macro invocation with `pub`
 --> src/main.rs:1:1
  |
1 | pub asyn fn test() {}
  | ^^^
  |
  = help: try adjusting the macro to put `pub` inside the invocation

error: expected one of `!` or `::`, found keyword `fn`
 --> src/main.rs:1:10
  |
1 | pub asyn fn test() {}
  |          ^^ expected one of `!` or `::`
