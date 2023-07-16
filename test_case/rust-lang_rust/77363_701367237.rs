rust
error[E0277]: the trait bound `Vec<_>: Type<_>` is not satisfied
 --> src/main.rs:3:15
  |
3 |         .bind(vec![]);
  |               ^^^^^^ the trait `Type<_>` is not implemented for `Vec<_>`
  |
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
