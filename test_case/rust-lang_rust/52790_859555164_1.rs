
error: expected one of `extern`, `fn`, or `unsafe`, found keyword `pub`
 --> src/lib.rs:3:7
  |
3 | async pub fn foo() -> impl Future<()> {todo!()}
  | ------^^^
  | |     |
  | |     expected one of `extern`, `fn`, or `unsafe`
  | help: visibility `pub` must come before `async`: `pub async`
