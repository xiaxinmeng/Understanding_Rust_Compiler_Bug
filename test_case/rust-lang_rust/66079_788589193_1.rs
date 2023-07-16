
$ rustc unknown-lint.rs  --crate-type lib
error[E0710]: an unknown tool name found in scoped lint: `xyz::my_lint`
 --> unknown-lint.rs:3:9
  |
3 | #![warn(xyz::my_lint)]
  |         ^^^
