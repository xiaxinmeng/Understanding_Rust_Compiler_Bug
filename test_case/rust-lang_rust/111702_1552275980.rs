plain
...................

failures:

---- src/option.rs - option::Option<T>::map_or_else (line 1162) stdout ----
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
  |
  |
3 | fn main() { #[allow(non_snake_case)] fn _doctest_main_src_option_rs_1162_0() {
  |                                      --------------------------------------- this function should return `Result` or `Option` to accept `?`
...
6 |    .map_or_else(|| std::fs::read_to_string("/etc/someconfig.conf"), Ok)?
  |                                                                        ^ cannot use the `?` operator in a function that returns `()`
  |
  = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`

error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
  |
  |
3 | fn main() { #[allow(non_snake_case)] fn _doctest_main_src_option_rs_1162_0() {
  |                                      --------------------------------------- this function should return `Result` or `Option` to accept `?`
7 |    .parse()?;
  |            ^ cannot use the `?` operator in a function that returns `()`
  |
  |
  = help: the trait `FromResidual<Result<Infallible, _>>` is not implemented for `()`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    src/option.rs - option::Option<T>::map_or_else (line 1162)

test result: FAILED. 4113 passed; 1 failed; 41 ignored; 0 measured; 0 filtered out; finished in 48.05s

error: doctest failed, to rerun pass `-p core --doc`
