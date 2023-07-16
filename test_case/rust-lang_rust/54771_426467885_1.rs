rust
error[E0277]: the trait bound `(): Bar` is not satisfied
 --> src/main.rs:6:5
  |
6 |     bar(|| { 5u8; })
  |     ^^^ the trait `Bar` is not implemented for `()`
  |
note: required by `bar`
 --> src/main.rs:4:1
  |
4 | fn bar<R: Bar>(_: impl Fn() -> R) {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
