
error[E0277]: the trait bound `[u8; 1]: Read` is not satisfied
 --> src/main.rs:8:16
  |
8 |     wants_read([0u8]);
  |     ---------- ^^^^^ the trait `Read` is not implemented for `[u8; 1]`
  |     |
  |     required by a bound introduced by this call
  |
  = help: the trait `Read` is implemented for `&[u8]`
note: required by a bound in `wants_read`
 --> src/main.rs:5:23
  |
5 | fn wants_read(_: impl Read) {}
  |                       ^^^^ required by this bound in `wants_read`
