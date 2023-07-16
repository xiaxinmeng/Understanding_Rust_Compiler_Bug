
error: expected one of `async`, `extern`, `fn`, or `unsafe`, found keyword `pub`
 --> src/main.rs:3:11
  |
3 | pub const pub fn toto() {}
  |     ------^^^
  |     |     |
  |     |     expected one of `async`, `extern`, `fn`, or `unsafe`
  |     help: visibility `pub` must come before `const`: `pub const`
