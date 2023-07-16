
error[E0308]: mismatched types
  --> src/lib.rs:10:24
   |
10 |     let _1: T::Type1 = x.into();
   |             --------   ^^^^^^^^ expected Trait::Type1, found Trait::Type2
   |             |
   |             expected due to this
   |
   = note: expected associated type `<T as Trait>::Type1`
              found associated type `<T as Trait>::Type2`
