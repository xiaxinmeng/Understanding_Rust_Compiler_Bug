
error[E0277]: the trait bound `u32: std::ops::Try` is not satisfied
  --> $DIR/try-unimplemented.rs:17:5
   |
17 |     3u32?
   |     ^^^^^ `?` cannot be applied to a value of type `u32`
   |
   = help: the trait `std::ops::Try` is not implemented for `u32`
   = note: required by `std::ops::Try::into_result`
