
error[E0119]: conflicting implementations of trait `Trait<()>` for type `()`
  --> dep/src/lib.rs:10:1
   |
9  | impl Trait<<() as Assoc>::Ty> for () {} // err
   | ------------------------------------ first implementation here
10 | impl Trait<()> for () {}
   | ^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `()`
