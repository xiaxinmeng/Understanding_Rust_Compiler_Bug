
error: can't qualify macro invocation with `pub`
 --> src/lib.rs:1:1
  |
1 | pub auto unsafe trait SingleCoreSend {}
  | ^^^
  |
  = help: try adjusting the macro to put `pub` inside the invocation

error: expected one of `!` or `::`, found `unsafe`
 --> src/lib.rs:1:10
  |
1 | pub auto unsafe trait SingleCoreSend {}
  |          ^^^^^^ expected one of `!` or `::` here

error: aborting due to 2 previous errors
