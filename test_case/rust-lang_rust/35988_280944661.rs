
rustc 1.15.1 (021bd294c 2017-02-08)
error[E0277]: the trait bound `[Box<E>]: std::marker::Sized` is not satisfied
 --> <anon>:2:7
  |
2 |     V([Box<E>])
  |       ^^^^^^^^^ the trait `std::marker::Sized` is not implemented for `[Box<E>]`
  |
  = note: `[Box<E>]` does not have a constant size known at compile-time
  = note: only the last field of a struct may have a dynamically sized type

error: aborting due to previous error
