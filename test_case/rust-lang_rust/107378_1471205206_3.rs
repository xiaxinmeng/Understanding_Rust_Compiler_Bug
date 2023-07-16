rust
error[[E0106]](https://doc.rust-lang.org/nightly/error_codes/E0106.html): missing lifetime specifier
 --> src/lib.rs:8:32
  |
8 | pub fn f(t: impl Trait<'_>) -> &str {
  |                                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
8 | pub fn f(t: impl Trait<'_>) -> &'static str {
  |                                 +++++++

For more information about this error, try `rustc --explain E0106`.
