
error[E0277]: the trait bound `T: Trait` is not satisfied
 --> src/main.rs:5:9
  |
5 |     bar(&t);
  |         ^^ the trait `Trait` is not implemented for `T`
  |
  = note: required for the cast to the object type `dyn Trait`
help: consider restricting type parameter `T`
  |
4 | fn foo<T: Trait>(t: T) {
  |         ^^^^^^^

error[E0277]: the trait bound `&T: Trait` is not satisfied
  --> src/main.rs:11:10
   |
11 |     bar2(&t);
   |          ^^ the trait `Trait` is not implemented for `&T`
...
14 | fn bar2<T: Trait>(t: T) {}
   |            ----- required by this bound in `bar2`

error[E0277]: the trait bound `T: Trait` is not satisfied
  --> src/main.rs:17:10
   |
14 | fn bar2<T: Trait>(t: T) {}
   |            ----- required by this bound in `bar2`
...
17 |     bar2(t);
   |          ^ the trait `Trait` is not implemented for `T`
   |
help: consider restricting type parameter `T`
   |
16 | fn foo3<T: Trait>(t: T) {
   |          ^^^^^^^

error[E0308]: mismatched types
  --> src/main.rs:21:9
   |
20 | fn foo4<T>(t: T) {
   |         - this type parameter
21 |     bar(t);
   |         ^
   |         |
   |         expected `&dyn Trait`, found type parameter `T`
   |         help: consider borrowing here: `&t`
   |
   = note:   expected reference `&dyn Trait`
           found type parameter `T`
