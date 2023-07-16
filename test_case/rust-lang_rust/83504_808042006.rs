
$ cargo +stage1 build
   Compiling issue83504 v0.1.0 (/home/omer/rust/issue83504)
error[E0658]: `let` expressions in this position are experimental
 --> src/main.rs:6:16
  |
6 |             if let json::JsonValue::Boolean(state) = v["active"] && true {
  |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
  = help: add `#![feature(let_chains)]` to the crate attributes to enable
  = help: you can write `matches!(<expr>, <pattern>)` instead of `let <pattern> = <expr>`

error: `let` expressions are not supported here
 --> src/main.rs:6:16
  |
6 |             if let json::JsonValue::Boolean(state) = v["active"] && true {
  |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: only supported directly without parentheses in conditions of `if`- and `while`-expressions, as well as in `let` chains within parentheses

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
error: could not compile `issue83504`

To learn more, run the command again with --verbose.
