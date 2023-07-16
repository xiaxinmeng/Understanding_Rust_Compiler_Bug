
    error[E0038]: the trait `Bar` cannot be made into an object
  --> ./test.rs:29:17
   |
29 |     play(&S1 as &Bar<A=S2>);
   |                 ^^^^^^^^^^ the trait `Bar` cannot be made into an object
   |
   = note: the trait cannot use `Self` as a type parameter in the supertraits or where-clauses

error[E0277]: the trait bound `Bar<A=S2>: Bar` is not satisfied
  --> ./test.rs:29:5
   |
29 |     play(&S1 as &Bar<A=S2>);
   |     ^^^^ the trait `Bar` is not implemented for `Bar<A=S2>`
   |
   = note: required by `play`

error[E0038]: the trait `Bar` cannot be made into an object
  --> ./test.rs:29:5
   |
29 |     play(&S1 as &Bar<A=S2>);
   |     ^^^^ the trait `Bar` cannot be made into an object
   |
   = note: the trait cannot use `Self` as a type parameter in the supertraits or where-clauses

error[E0038]: the trait `Bar` cannot be made into an object
  --> ./test.rs:29:10
   |
29 |     play(&S1 as &Bar<A=S2>);
   |          ^^^ the trait `Bar` cannot be made into an object
   |
   = note: the trait cannot use `Self` as a type parameter in the supertraits or where-clauses
   = note: required because of the requirements on the impl of `std::ops::CoerceUnsized<&Bar<A=S2>>` for `&S1`

error: aborting due to 4 previous errors
