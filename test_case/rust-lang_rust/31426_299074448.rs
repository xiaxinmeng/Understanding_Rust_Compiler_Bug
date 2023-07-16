
error[E0277]: the trait bound `[u8]: std::marker::Sized` is not satisfied
 --> test.rs:1:1
  |
1 | / fn test(t: &([u8], [u16])) -> &[u16] {
2 | |     &t.1
3 | | }
  | |_^ the trait `std::marker::Sized` is not implemented for `[u8]`
  |
  = note: `[u8]` does not have a constant size known at compile-time
  = note: tuple elements must have `Sized` type
