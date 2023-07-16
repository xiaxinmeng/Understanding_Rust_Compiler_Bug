
error[E0277]: the trait bound `(): Trait` is not satisfied
  --> src/main.rs:11:17
   |
11 | fn example() -> impl Trait {
   |                 ^^^^^^^^^^ the trait `Trait` is not implemented for `()`
   |
   = help: the trait `Trait` is implemented for `Struct`

