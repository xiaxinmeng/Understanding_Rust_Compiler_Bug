rust
error[E0308]: mismatched types
 --> src/main.rs:1:21
  |
1 | static FOO: [u32] = [0, 1, 2];
  |                     ^^^^^^^^^ expected slice, found array of 3 elements
  |
  = note: expected type `[u32]`
             found type `[u32; 3]`

error[E0277]: the trait bound `[u32]: std::marker::Sized` is not satisfied
 --> src/main.rs:1:21
  |
1 | static FOO: [u32] = [0, 1, 2];
  |                     ^^^^^^^^^ `[u32]` does not have a constant size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `[u32]`
  = note: constant expressions must have a statically known size
