
warning: literal non-ASCII character detected
 --> src/main.rs:3:13
  |
3 |     let _ = "Üben!";
  |             ^^^^^^^ help: consider replacing the string with: `"\u{dc}ben!"`
  |
note: lint level defined here
 --> src/main.rs:1:8
  |
1 | #[warn(clippy::non_ascii_literal)]
  |        ^^^^^^^^^^^^^^^^^^^^^^^^^
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#non_ascii_literal

warning: literal non-ASCII character detected
 --> src/main.rs:4:12
  |
4 |     print!("Üben!");
  |            ^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#non_ascii_literal
