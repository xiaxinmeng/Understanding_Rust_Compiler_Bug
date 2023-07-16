

error[E0277]: the trait bound `[u8]: std::marker::Sized` is not satisfied in type `std::path::Path`
 --> <anon>:3:6
  |
3 | fn f(p: Path) { }
  |      ^ within type `std::path::Path`, the trait `std::marker::Sized` is not implemented for `[u8]`
  |
  = note: `[u8]` does not have a constant size known at compile-time
  = note: required because it appears within the type `std::path::Path`
  = note: all local variables must have a statically known size
