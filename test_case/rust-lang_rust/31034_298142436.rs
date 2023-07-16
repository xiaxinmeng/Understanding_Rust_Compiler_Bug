
rustc 1.18.0-nightly (94e884b63 2017-04-27)
error[E0277]: the trait bound `f64: Mul` is not satisfied
  --> <anon>:10:13
   |
10 |     <f64 as Mul>::Output::zero();
   |             ^^^^^^^^^^^^^^^^^^ the trait `Mul` is not implemented for `f64`

error: no associated item named `zero` found for type `_` in the current scope
  --> <anon>:10:5
   |
10 |     <f64 as Mul>::Output::zero();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: items from traits can only be used if the trait is implemented and in scope; the following trait defines an item `zero`, perhaps you need to implement it:
   = help: candidate #1: `Zero`

error: aborting due to 2 previous errors
