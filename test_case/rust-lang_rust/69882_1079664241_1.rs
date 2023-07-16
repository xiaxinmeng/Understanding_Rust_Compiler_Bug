
error[E0277]: the trait bound `(): Trait` is not satisfied
 --> src/lib.rs:3:15  
  |
3 | fn magic() -> impl Trait {
  |               ^^^^^^^^^^ the trait `Trait` is not implemented for `()`
