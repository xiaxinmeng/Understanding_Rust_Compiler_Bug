
error[E0277]: the trait bound `[u8]: std::marker::Sized` is not satisfied
 --> src/logic.rs:5:5
  |
5 |     product_path: Path,
  |     ^^^^^^^^^^^^^^^^^^ the trait `std::marker::Sized` is not implemented for `[u8]`
  |
  = note: `[u8]` does not have a constant size known at compile-time
  = note: required because it appears within the type `std::path::Path`
  = note: only the last field of a struct may have a dynamically sized type
