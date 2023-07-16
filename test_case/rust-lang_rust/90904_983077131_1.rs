
error[E0277]: cannot add `&()` to `&usize`
 --> src/main.rs:9:11
  |
9 |         b + d
  |           ^ no implementation for `&usize + &()`
  |
  = help: the trait `Add<&()>` is not implemented for `&usize`

error[E0277]: the trait bound `(): FromStr` is not satisfied
    --> src/main.rs:7:14
     |
7    |             .parse()
     |              ^^^^^ the trait `FromStr` is not implemented for `()`
     |
note: required by a bound in `core::str::<impl str>::parse`
    --> /home/antonok/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/mod.rs:2247:21
     |
2247 |     pub fn parse<F: FromStr>(&self) -> Result<F, F::Err> {
     |                     ^^^^^^^ required by this bound in `core::str::<impl str>::parse`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `regression` due to 2 previous errors
