
error: variables in the condition are not mutated in the loop body
 --> src/main.rs:4:11
  |
4 |     while i < len {
  |           ^^^^^^^
  |
  = note: this may lead to an infinite or to a never running loop
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#while_immutable_condition
  = note: `#[deny(clippy::while_immutable_condition)]` on by default
