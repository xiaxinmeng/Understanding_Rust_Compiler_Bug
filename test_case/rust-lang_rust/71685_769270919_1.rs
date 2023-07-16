
error[E0277]: the trait bound `(): FromStr` is not satisfied
 --> src/lib.rs:2:15
  |
2 |     let s = s.parse().map_err(|_| ())?;
  |               ^^^^^ the trait `FromStr` is not implemented for `()`
