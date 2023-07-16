
error[E0658]: the precise format of `Fn`-family traits' type parameters is subject to change
 --> src/lib.rs:3:18
  |
3 | fn search<'a, T: Fn<(char,)>>(query: &T, contents: &'a str) -> Vec<&'a str> {
  |                  ^^ help: use parenthetical notation instead: `Fn(char,) -> ()`
  |
  = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
