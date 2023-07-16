
error[E0119]: conflicting implementations of trait `OtherTrait` for type `()`:
  --> $DIR/reservation-impl-coherence-conflict.rs:13:1
   |
LL | impl OtherTrait for () {}
   | ---------------------- first implementation here
LL | impl<T: MyTrait> OtherTrait for T {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `()`
   |
   = note: this impl is reserved

error: aborting due to previous error
